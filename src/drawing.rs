use crate::Color;
use minifb::Window;
use rand::Rng;
use vecmath::{vec3_cross, Vector3};
use wavefront::Obj;

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub fn draw_line(buffer: &mut Vec<u32>, window: &Window, a: &Position, b: &Position, color: Color) {
    let mut x0 = a.x;
    let mut x1 = b.x;
    let mut y0 = a.y;
    let mut y1 = b.y;
    let mut steep = false;

    // Number of points should be calculated based off vertical distance if line is steep
    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    // Always draw left to right
    if x1 < x0 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    for x in x0..x1 {
        // Step
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = (y0 as f32 * (1. - t) as f32 + y1 as f32 * t) as u32;
        if steep {
            set_pixel(window, buffer, y, x, color);
        } else {
            set_pixel(window, buffer, x, y, color);
        }
    }
}

pub fn set_pixel(window: &Window, buffer: &mut Vec<u32>, x: u32, y: u32, color: Color) {
    let width = window.get_size().0;
    let index = (y * width as u32 + x) as usize;
    if index < buffer.len() {
        buffer[index] = color.as_u32();
    }
}

// Old method that used line sweeping
pub fn triangle_line_sweep(
    buffer: &mut Vec<u32>,
    window: &Window,
    a: Position,
    b: Position,
    c: Position,
    color: Color,
) {
    // Get the top, middle,  bottom points of the triangle
    let mut vertices = [a.clone(), b.clone(), c.clone()];
    vertices.sort_by_key(|pos| pos.y);
    let top = vertices[0].clone();
    let middle = vertices[1].clone();
    let bottom = vertices[2].clone();

    println!("top {:?} middle {:?} bottom {:?}", top, middle, bottom);

    // Linesweep from  top.y to bottom.y
    for y in top.y..bottom.y {
        // x1 and x2 will be the intersection points
        // with the sides of the triangle
        let mut x1 = Position {
            x: 0 as u32,
            y: y as u32,
        };
        let mut x2 = Position {
            x: 0 as u32,
            y: y as u32,
        };
        // For each x along the window width, check which side it intersects
        // and save the corresponding x
        for x in 0..window.get_size().0 {
            let p = Position {
                x: x as u32,
                y: y as u32,
            };
            // Once the middle point, vertically wise, is crossed
            // check for intersection between middle point and botton point
            if y < middle.y {
                // top - middle
                if is_on_line(&p, &top, &middle) {
                    x1.x = x as u32;
                }
            } else {
                // bottom - middle
                if is_on_line(&p, &bottom, &middle) {
                    x1.x = x as u32;
                }
            }
            // top - bottom
            if is_on_line(&p, &top, &bottom) {
                x2.x = x as u32;
            }
        }
        // Draw line between the two intersection point
        draw_line(buffer, window, &x1, &x2, color);
    }

    // Limits of the triangle, DEBUG
    draw_line(buffer, &window, &a, &b, Color::red());
    draw_line(buffer, &window, &b, &c, Color::red());
    draw_line(buffer, &window, &c, &a, Color::red());
}

// Computes the barycentric coordinates of point P based on the triangle ABC
fn barycentric(a: &Position, b: &Position, c: &Position, p: &Position) -> Vector3<f32> {
    let s: Vector3<f32> = [
        b.x as f32 - a.x as f32,
        c.x as f32 - a.x as f32,
        a.x as f32 - p.x as f32,
    ];
    let t: Vector3<f32> = [
        b.y as f32 - a.y as f32,
        c.y as f32 - a.y as f32,
        a.y as f32 - p.y as f32,
    ];
    let cross = vec3_cross(s, t);
    if cross[2].abs() < 1. {
        // If cross[2].abs() < 1, cross[2] is 0 and the triangle is degenerate
        return [-1., 1., 1.];
    }
    let u: Vector3<f32> = [
        1. - (cross[0] + cross[1]) / cross[2],
        cross[1] / cross[2],
        cross[0] / cross[2],
    ];
    return u;
}

pub fn triangle(
    buffer: &mut Vec<u32>,
    window: &Window,
    a: Position,
    b: Position,
    c: Position,
    color: Color,
) {
    // Find bounding box
    let min_x = a.x.min(b.x).min(c.x);
    let max_x = a.x.max(b.x).max(c.x);

    let min_y = a.y.min(b.y).min(c.y);
    let max_y = a.y.max(b.y).max(c.y);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = Position { x, y };
            // Check if p is inside the triangle
            let barycentric = barycentric(&a, &b, &c, &p);
            let u = barycentric[0];
            let v = barycentric[1];
            let is_inside = u > 0. && v > 0. && u + v < 1.;
            if is_inside {
                set_pixel(window, buffer, x, y, color);
            }
        }
    }
}

// Check if c in on a line formed by a and b
fn is_on_line(a: &Position, b: &Position, check: &Position) -> bool {
    // Horizontal line
    if a.y == b.y && a.y == check.y {
        return true;
    }
    // Vertical line
    if a.x == b.x && a.x == check.x {
        return true;
    }
    // line of equation y = mx + p
    let m = ((a.y as f32 - b.y as f32) / (a.x as f32 - b.x as f32)) as f32;
    let p = (a.y as f32 - m * a.x as f32) as f32;
    return check.y == (m * check.x as f32 + p) as u32;
}

pub fn draw_wireframe(window: &Window, buffer: &mut Vec<u32>, model: Obj) {
    let (width, height) = window.get_size();
    for [a, b, c] in model.triangles() {
        let x_a = ((a.position()[0] + 1.) * width as f32 / 2.) as u32;
        let y_a = ((a.position()[1] + 1.) * height as f32 / 2.) as u32;
        let x_b = ((b.position()[0] + 1.) * width as f32 / 2.) as u32;
        let y_b = ((b.position()[1] + 1.) * height as f32 / 2.) as u32;
        let x_c = ((c.position()[0] + 1.) * width as f32 / 2.) as u32;
        let y_c = ((c.position()[1] + 1.) * height as f32 / 2.) as u32;
        let a = Position { x: x_a, y: y_a };
        let b = Position { x: x_b, y: y_b };
        let c = Position { x: x_c, y: y_c };
        // Generate random values for red, green, and blue components
        let mut rng = rand::thread_rng();
        let red: u8 = rng.gen();
        let green: u8 = rng.gen();
        let blue: u8 = rng.gen();
        triangle(
            buffer,
            window,
            a,
            b,
            c,
            Color::from_u8_rgb(red, green, blue),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::drawing::{is_on_line, Position};

    #[test]
    fn vertical_line() {
        // Vertical line
        let a = Position { x: 6, y: 0 };
        let b = Position { x: 6, y: 100 };
        let c = Position { x: 6, y: 90 };
        assert!(is_on_line(&a, &b, &c));
    }

    #[test]
    fn horizontal_line() {
        // Horizontal line
        let a = Position { x: 6, y: 250 };
        let b = Position { x: 85, y: 250 };
        let c = Position { x: 300, y: 250 };
        assert!(is_on_line(&a, &b, &c));
    }
}
