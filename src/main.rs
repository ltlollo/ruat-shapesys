extern crate sfml;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const OFF: f32 = 10.0;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{RenderWindow, RenderTarget, Color, Vertex};

mod lang;
use lang::*;
use lang::geom::*;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(WIDTH, HEIGHT, 32),
                                       "shapesys",
                                       Close,
                                       &ContextSettings::default())
                         .expect("Cannot create a Render Window.");

    window.clear(&Color::black());
    window.display();

    let f = Rule::from_bytes(b"AannBbnnCcnnDdnn>aBb,bCc,cDd,dAa,abcd").unwrap();
    let s = Rule::from_bytes(b"AavBbvCcv>cAa,aBb,bCc").unwrap();

    let mut g = Grammar::new(&[f, s]).unwrap();

    let first_shape: Vec<Shape> = vec![vec![Vertex::new_with_pos(&Vector2f {
                                                x: 0f32 + OFF,
                                                y: 0f32 + OFF,
                                            }),
                                            Vertex::new_with_pos(&Vector2f {
                                                x: 0f32 + OFF,
                                                y: 1000f32 + OFF,
                                            }),
                                            Vertex::new_with_pos(&Vector2f {
                                                x: 1000f32 + OFF,
                                                y: 1000f32 + OFF,
                                            }),
                                            Vertex::new_with_pos(&Vector2f {
                                                x: 1000f32 + OFF,
                                                y: 0f32 + OFF,
                                            })]];
    let shapes = g.iterate(&first_shape, 10);
    draw_shapes(&mut window, &shapes);
    while window.is_open() {
        for event in window.events() {
            match event {
                event::KeyPressed{code, ..} => {
                    match code {
                        Key::Escape => {
                            window.close();
                        }
                        Key::S => {
                            if let Some(img) = window.capture() {
                                let fname = g.as_string() + ".png";
                                img.save_to_file(&fname);
                            }
                        }
                        _ => (),
                    }
                }
                event::Closed => window.close(),
                _ => (),
            }
        }
        window.display();
    }
}
