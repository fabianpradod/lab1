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

// Function to fill a polygon using the scanline algorithm
pub fn fill_polygon(canvas: &mut WindowCanvas, vertices: &[Point2D]) -> Result<(), Box<dyn std::error::Error>> {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    
    let mut min_y = vertices[0].y;
    let mut max_y = vertices[0].y;
    
    for vertex in vertices {
        if vertex.y < min_y { min_y = vertex.y; }
        if vertex.y > max_y { max_y = vertex.y; }
    }
    
    for y in min_y..=max_y {
        // Find all intersection points where this horizontal line touches the polygon edges
        let mut intersections = Vec::new();

        for i in 0..vertices.len() {
            let current = vertices[i];
            let next = vertices[(i + 1) % vertices.len()];
            
            // Check if this edge intersects with our horizontal line at y
            if (current.y <= y && next.y > y) || (current.y > y && next.y <= y) {
                // Calculate the x coordinate of the intersection
                let intersection_x = current.x + (y - current.y) * (next.x - current.x) / (next.y - current.y);
                intersections.push(intersection_x);
            }
        }
        
        // Sort intersection points from left to right
        intersections.sort();
        
        // Fill between pairs of intersections
        for chunk in intersections.chunks(2) {
            if chunk.len() == 2 {
                let start_x = chunk[0];
                let end_x = chunk[1];
                for x in start_x..=end_x {
                    let point = Point::new(x, y);
                    canvas.draw_point(point)?;
                }
            }
        }
    }
    
    Ok(())
}