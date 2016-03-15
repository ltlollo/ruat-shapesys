extern crate sfml;

use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, RenderStates, Vertex};
pub type Shape = Vec<Vector2f>;

pub fn mid(f: &Vector2f, s: &Vector2f) -> Vector2f {
    Vector2f {
        x: (f.x + s.x) / 2f32,
        y: (f.y + s.y) / 2f32,
    }
}
pub fn div_vec(f: &Vector2f, s: &Vector2f, of: f32, n: f32) -> Vector2f {
    Vector2f {
        x: (s.x - f.x) * (of / n) + f.x,
        y: (s.y - f.y) * (of / n) + f.y,
    }
}

pub fn draw_shapes(window: &mut RenderWindow,
                   shapes: &Vec<Shape>,
                   rs: &mut RenderStates) {
    use sfml::graphics::PrimitiveType::sfLines as Lines;
    use sfml::graphics::PrimitiveType::sfPoints as Points;
    for shape in shapes.iter() {
        let mut seg: [Vertex; 2];
        if shape.len() == 1 {
            seg = [Vertex::new_with_pos(&shape[0]),
                   Vertex::new_with_pos(&shape[0])];
            window.draw_primitives(&seg[0..1], Points, rs);
            continue;
        }
        for i in 0..shape.len() - 1 {
            seg = [Vertex::new_with_pos(&shape[i]),
                   Vertex::new_with_pos(&shape[i + 1])];
            window.draw_primitives(&seg[..], Lines, rs);
        }
        if shape.len() > 2 {
            seg = [Vertex::new_with_pos(&shape[shape.len() - 1]),
                   Vertex::new_with_pos(&shape[0])];
            window.draw_primitives(&seg[..], Lines, rs);
        }
    }
}

pub fn calc_center(shape: &Shape) -> Vector2f {
    let mut c = Vector2f { x: 0f32, y: 0f32 };
    for v in shape.iter() {
        c.x = c.x + v.x;
        c.y = c.y + v.y;
    }
    c.x = c.x / shape.len() as f32;
    c.y = c.y / shape.len() as f32;
    c
}
