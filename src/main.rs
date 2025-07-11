extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use std::time::Duration;

// Struct to represent a 2D point
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

// Fun to implement the Point2D struct
impl Point2D {
    // Constructor
    pub fn new(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }
    
    // Convert Point2D to SDL2 format
    pub fn to_sdl_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// Function to draw the outline of a polygon given the list of vertices
pub fn draw_polygon_outline(canvas: &mut WindowCanvas, vertices: &[Point2D]) -> Result<(), Box<dyn std::error::Error>> {
    // Check if we have at least 4 points
    if vertices.len() < 4 {
        println!("Alerta: Se necesitan por lo menos 4 puntos");
        return Ok(());
    }
    
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    // Draw lines between consecutive points
    for i in 0..vertices.len() {
        // Since the last point connects back to the first point,
        // Use (%) to wrap around: when i is the last index, next i becomes 0 (first point)
        let next_i = (i + 1) % vertices.len();
        
        let current_point = vertices[i].to_sdl_point();
        let next_point = vertices[next_i].to_sdl_point();
        
        canvas.draw_line(current_point, next_point)?;
    }
    Ok(())
}