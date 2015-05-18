extern crate sfml;

use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, Vertex, PrimitiveType};
pub type Shape = Vec<Vertex>;

pub fn mid(f: &Vertex, s: &Vertex) -> Vertex {
    Vertex::new_with_pos(&Vector2f{
        x: (f.position.x + s.position.x)/2f32,
        y: (f.position.y + s.position.y)/2f32
    })
}

pub fn div_seg(f: &Vertex, s: &Vertex, of: f32, n: f32) -> Vertex {
    let min_x = f.position.x.min(s.position.x);
    let max_x = f.position.x.max(s.position.x);
    let min_y = f.position.y.min(s.position.y);
    let max_y = f.position.y.max(s.position.y);
    let x  = if f.position.x == min_x { (max_x-min_x)*    of/n + min_x } else {
                                        (max_x-min_x)*(n-of)/n + min_x };
    let y  = if f.position.y == min_y { (max_y-min_y)*    of/n + min_y } else {
                                        (max_y-min_y)*(n-of)/n + min_y };
    Vertex::new_with_pos(&Vector2f{ x: x, y: y})
}

pub fn draw_shapes(window: &mut RenderWindow, shapes: &Vec<Shape>) {
    for shape in shapes.iter() {
        if shape.len() == 1 {
            window.draw_primitives(&shape[0..1], PrimitiveType::Points);
            continue;
        }
        for i in 0..shape.len()-1 {
             window.draw_primitives(&shape[i..i+2], PrimitiveType::Lines);
        } if shape.len() > 2 {
            let last: [Vertex; 2] = [shape[shape.len()-1], shape[0]];
            window.draw_primitives(&last[..], PrimitiveType::Lines);
       }
    }
}

pub fn calc_center(shape: &Shape) -> Vertex {
    let mut c = Vector2f::new(0f32, 0f32);
    for v in shape.iter() {
        c.x = c.x + v.position.x;
        c.y = c.y + v.position.y;
    }
    c.x = c.x/shape.len() as f32;
    c.y = c.y/shape.len() as f32;
    Vertex::new_with_pos(&c)
}
