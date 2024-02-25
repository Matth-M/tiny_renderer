use crate::Color;
use minifb::Window;
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

pub fn triangle(
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
        // For each x along the window, check which side it intersect
        //  and save the corresponding x
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
    draw_line(buffer, &window, &a, &b, Color::Red);
    draw_line(buffer, &window, &b, &c, Color::Red);
    draw_line(buffer, &window, &c, &a, Color::Red);
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

pub fn draw_wireframe(window: &Window, buffer: &mut Vec<u32>, model: Obj, color: Color) {
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
        triangle(buffer, window, a, b, c, color)
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
