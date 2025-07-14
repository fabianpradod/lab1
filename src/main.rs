extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }
    
    pub fn to_sdl_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let pixel_count = (width * height) as usize;
        FrameBuffer {
            width,
            height,
            pixels: vec![Color::RGB(0, 0, 0); pixel_count],
        }
    }
    
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }
    
    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
    
    pub fn draw_to_canvas(&self, canvas: &mut WindowCanvas) -> Result<(), Box<dyn std::error::Error>> {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let color = self.pixels[index];
                canvas.set_draw_color(color);
                canvas.draw_point(Point::new(x, y))?;
            }
        }
        Ok(())
    }
    
    pub fn save_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::BufWriter;
        
        let file = File::create(filename)?;
        let ref mut w = BufWriter::new(file);
        
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        
        let mut writer = encoder.write_header()?;
        
        let mut rgb_data = Vec::new();
        for pixel in &self.pixels {
            rgb_data.push(pixel.r);
            rgb_data.push(pixel.g);
            rgb_data.push(pixel.b);
        }
        
        writer.write_image_data(&rgb_data)?;
        Ok(())
    }
}

pub fn draw_polygon_outline(framebuffer: &mut FrameBuffer, vertices: &[Point2D]) -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..vertices.len() {
        let next_i = (i + 1) % vertices.len();
        let current_point = vertices[i];
        let next_point = vertices[next_i];
        draw_line(framebuffer, current_point, next_point, Color::RGB(255, 255, 255));
    }
    Ok(())
}

pub fn fill_polygon(framebuffer: &mut FrameBuffer, vertices: &[Point2D]) -> Result<(), Box<dyn std::error::Error>> {
    let mut min_y = vertices[0].y;
    let mut max_y = vertices[0].y;
    
    for vertex in vertices {
        if vertex.y < min_y { min_y = vertex.y; }
        if vertex.y > max_y { max_y = vertex.y; }
    }
    
    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..vertices.len() {
            let current = vertices[i];
            let next = vertices[(i + 1) % vertices.len()];
            
            if (current.y <= y && next.y > y) || (current.y > y && next.y <= y) {
                let intersection_x = current.x + (y - current.y) * (next.x - current.x) / (next.y - current.y);
                intersections.push(intersection_x);
            }
        }
        
        intersections.sort();
        
        for chunk in intersections.chunks(2) {
            if chunk.len() == 2 {
                let start_x = chunk[0];
                let end_x = chunk[1];
                for x in start_x..=end_x {
                    framebuffer.set_pixel(x, y, Color::RGB(255, 0, 0));
                }
            }
        }
    }
    
    Ok(())
}

fn draw_line(framebuffer: &mut FrameBuffer, start: Point2D, end: Point2D, color: Color) {
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;
    
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    
    loop {
        framebuffer.set_pixel(x0, y0, color);
        
        if x0 == x1 && y0 == y1 { break; }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let polygons = vec![
        // Polygon 1
        vec![
            Point2D::new(165, 380), Point2D::new(185, 360), Point2D::new(180, 330), 
            Point2D::new(207, 345), Point2D::new(233, 330), Point2D::new(230, 360), 
            Point2D::new(250, 380), Point2D::new(220, 385), Point2D::new(205, 410), 
            Point2D::new(193, 383)
        ],
        // Polygon 2
        vec![
            Point2D::new(321, 335), Point2D::new(288, 286), Point2D::new(339, 251), Point2D::new(374, 302)
        ],
        // Polygon 3
        vec![
            Point2D::new(377, 249), Point2D::new(411, 197), Point2D::new(436, 249)
        ],
        // Polygon 4
        vec![
            Point2D::new(413, 177), Point2D::new(448, 159), Point2D::new(502, 88), Point2D::new(553, 53), 
            Point2D::new(535, 36), Point2D::new(676, 37), Point2D::new(660, 52), Point2D::new(750, 145), 
            Point2D::new(761, 179), Point2D::new(672, 192), Point2D::new(659, 214), Point2D::new(615, 214), 
            Point2D::new(632, 230), Point2D::new(580, 230), Point2D::new(597, 215), Point2D::new(552, 214), 
            Point2D::new(517, 144), Point2D::new(466, 180)
        ],
        // Polygon 5
        vec![
            Point2D::new(682, 175), Point2D::new(708, 120), Point2D::new(735, 148), Point2D::new(739, 170)
        ],
    ];
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Polygons", 800, 600).position_centered().build()?;
    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut framebuffer = FrameBuffer::new(800, 600);
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        
        framebuffer.clear(Color::RGB(0, 0, 0));
        
        for polygon in &polygons {
            fill_polygon(&mut framebuffer, polygon)?;
            draw_polygon_outline(&mut framebuffer, polygon)?;
        }
        
        // Save PNG on first frame
        static mut SAVED: bool = false;
        unsafe {
            if !SAVED {
                framebuffer.save_png("polygons.png")?;
                println!("PNG saved as polygons.png");
                SAVED = true;
            }
        }
        
        framebuffer.draw_to_canvas(&mut canvas)?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}