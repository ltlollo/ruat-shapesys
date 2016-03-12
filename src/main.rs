extern crate sfml;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const OFF: f32 = 0.0;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, window_style};
use sfml::window::Key;
use sfml::graphics::{RenderWindow, RenderTarget, RenderStates, Color};

mod lang;
use lang::*;
use lang::geom::*;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(WIDTH, HEIGHT, 32),
                                       "shapesys",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
                         .expect("Cannot create a Render Window.");

    window.clear(&Color::black());
    window.display();

    let f = Rule::from_bytes(b"ABCD>AB.,BC.,CD.,DA.").unwrap();
    let s = Rule::from_bytes(b"AaBnnnnncnCndnnnnn>acd,Aad,aBc,dcC").unwrap();

    let mut g = Grammar::new(&[f, s]).unwrap();

    let first_shape: Vec<Shape> = vec![vec![Vector2f {
                                                x: 0f32 + OFF,
                                                y: 0f32 + OFF,
                                            },
                                            Vector2f {
                                                x: 0f32 + OFF,
                                                y: HEIGHT as f32 - OFF,
                                            },
                                            Vector2f {
                                                x: WIDTH as f32 - OFF,
                                                y: HEIGHT as f32 - OFF,
                                            },
                                            Vector2f {
                                                x: WIDTH as f32 - OFF,
                                                y: 0f32 + OFF,
                                            }]];
    let shapes = g.iterate(&first_shape, 7);
    let mut rs = RenderStates::default();
    draw_shapes(&mut window, &shapes, &mut rs);
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
