use crate::Color;
use minifb::Window;
use vecmath::{vec3_cross, vec3_dot, vec3_normalized, Vector3};
use wavefront::{Obj, Vertex};

#[derive(Clone, PartialEq, Debug)]
pub struct ScreenPosition {
    pub x: u32,
    pub y: u32,
}

/// Draw a line between a and b.
pub fn draw_line(
    buffer: &mut Vec<u32>,
    window: &Window,
    a: &ScreenPosition,
    b: &ScreenPosition,
    color: Color,
) {
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
        let y = (y0 as f32 * (1. - t) + y1 as f32 * t) as u32;
        if steep {
            set_pixel(window, buffer, y, x, color);
        } else {
            set_pixel(window, buffer, x, y, color);
        }
    }
}

/// Set pixel with color in the buffer
pub fn set_pixel(window: &Window, buffer: &mut Vec<u32>, x: u32, y: u32, color: Color) {
    let width = window.get_size().0;
    // Use the complementary of the coordinates to render the right way
    let index = (y * width as u32 + x) as usize;
    if index < buffer.len() {
        buffer[index] = color.as_u32();
    }
}

#[allow(dead_code)]
/// Draw triangle filled triangle based on 3 points using the line sweeping algorithm.
pub fn triangle_line_sweep(
    buffer: &mut Vec<u32>,
    window: &Window,
    a: ScreenPosition,
    b: ScreenPosition,
    c: ScreenPosition,
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
        let mut x1 = ScreenPosition { x: 0_u32, y };
        let mut x2 = ScreenPosition { x: 0_u32, y };
        // For each x along the window width, check which side it intersects
        // and save the corresponding x
        for x in 0..window.get_size().0 {
            let p = ScreenPosition { x: x as u32, y };
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
    outline_triangle(buffer, window, &a, &b, &c, color);
}

/// Computes the barycentric coordinates of point P based on the triangle ABC
fn barycentric(
    a: &ScreenPosition,
    b: &ScreenPosition,
    c: &ScreenPosition,
    p: &ScreenPosition,
) -> Vector3<f32> {
    let s: Vector3<f32> = [
        a.x as f32 - b.x as f32,
        a.x as f32 - c.x as f32,
        p.x as f32 - a.x as f32,
    ];
    let t: Vector3<f32> = [
        a.y as f32 - b.y as f32,
        a.y as f32 - c.y as f32,
        p.y as f32 - a.y as f32,
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

    u
}

/// Draw outline of triangle
fn outline_triangle(
    buffer: &mut Vec<u32>,
    window: &Window,
    a: &ScreenPosition,
    b: &ScreenPosition,
    c: &ScreenPosition,
    color: Color,
) {
    draw_line(buffer, window, a, b, color);
    draw_line(buffer, window, a, c, color);
    draw_line(buffer, window, c, b, color);
}

fn is_inside_triangle(
    a: &ScreenPosition,
    b: &ScreenPosition,
    c: &ScreenPosition,
    p: &ScreenPosition,
) -> bool {
    let barycentric = barycentric(a, b, c, &p);
    let u = barycentric[0];
    let v = barycentric[1];
    return u > 0. && v > 0. && u + v < 1.;
}

pub fn fill_triangle(
    buffer: &mut Vec<u32>,
    window: &Window,
    a: &ScreenPosition,
    b: &ScreenPosition,
    c: &ScreenPosition,
    color: Color,
) {
    // Find bounding box
    let min_x = a.x.min(b.x).min(c.x);
    let max_x = a.x.max(b.x).max(c.x);

    let min_y = a.y.min(b.y).min(c.y);
    let max_y = a.y.max(b.y).max(c.y);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = ScreenPosition { x, y };
            // Check if p is inside the triangle
            if is_inside_triangle(a, b, c, &p) {
                set_pixel(window, buffer, x, y, color);
            }
        }
    }
}

#[allow(dead_code)]
/// Check if c in on a line formed by a and b
fn is_on_line(a: &ScreenPosition, b: &ScreenPosition, check: &ScreenPosition) -> bool {
    // Horizontal line
    if a.y == b.y && a.y == check.y {
        return true;
    }
    // Vertical line
    if a.x == b.x && a.x == check.x {
        return true;
    }
    // line of equation y = mx + p
    let m = (a.y as f32 - b.y as f32) / (a.x as f32 - b.x as f32);
    let p = a.y as f32 - m * a.x as f32;
    check.y == (m * check.x as f32 + p) as u32
}

//
fn get_intensity(worlds_coords: [&Vertex; 3], light_direction: [f32; 3]) -> f32 {
    let a = worlds_coords[0].position();
    let b = worlds_coords[1].position();
    let c = worlds_coords[2].position();
    let ab = vecmath::vec3_sub(b, a);
    let ac = vecmath::vec3_sub(c, a);
    let normal = vec3_cross(ab, ac);
    let normal = vec3_normalized(normal);
    vec3_dot(normal, light_direction)
}

fn convert_to_screen_coordinates(v: &Vertex, window: &Window) -> ScreenPosition {
    let (width, height) = window.get_size();
    let x = ((v.position()[0] + 1.) * width as f32 / 2.) as u32;
    let y = ((v.position()[1] + 1.) * height as f32 / 2.) as u32;
    ScreenPosition { x, y }
}

/// Set buffer to render the model
pub fn render_model(window: &Window, buffer: &mut Vec<u32>, model: &Obj) {
    // Iterate through models triangles and draw them
    let light_direction = [0., 0., 1.];

    for [a, b, c] in model.triangles() {
        let intensity = get_intensity([&a, &b, &c], light_direction);
        if intensity > 0. {
        let a = convert_to_screen_coordinates(&a, window);
        let b = convert_to_screen_coordinates(&b, window);
        let c = convert_to_screen_coordinates(&c, window);
            fill_triangle(
                buffer,
                window,
                &a,
                &b,
                &c,
                Color::from_u8_rgb(
                    (intensity * 255.) as u8,
                    (intensity * 255.) as u8,
                    (intensity * 255.) as u8,
                ),
            );
            outline_triangle(buffer, window, &a, &b, &c, Color::white());
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_line() {
        // Vertical line
        let a = ScreenPosition { x: 6, y: 0 };
        let b = ScreenPosition { x: 6, y: 100 };
        let c = ScreenPosition { x: 6, y: 90 };
        assert!(is_on_line(&a, &b, &c));
    }

    #[test]
    fn horizontal_line() {
        // Horizontal line
        let a = ScreenPosition { x: 6, y: 250 };
        let b = ScreenPosition { x: 85, y: 250 };
        let c = ScreenPosition { x: 300, y: 250 };
        assert!(is_on_line(&a, &b, &c));
    }

    #[test]
    fn test_is_inside_triangle() {
        let a = ScreenPosition { x: 0, y: 0 };
        let b = ScreenPosition { x: 10, y: 0 };
        let c = ScreenPosition { x: 0, y: 20 };
        let inside = ScreenPosition { x: 1, y: 1 };
        assert!(is_inside_triangle(&a, &b, &c, &inside));

        let outside = ScreenPosition { x: 46, y: 1 };
        assert!(!is_inside_triangle(&a, &b, &c, &outside));
    }
}
