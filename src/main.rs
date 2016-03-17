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

fn process(g: &mut Grammar, niter: u8) {
    let mut window = RenderWindow::new(VideoMode::new_init(WIDTH, HEIGHT, 32),
                                       "shapesys",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
                         .expect("Cannot create a Render Window.");

    window.clear(&Color::black());
    window.display();
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
    let mut rs = RenderStates::default();
    let shapes = g.iterate(&mut window, &mut rs, &first_shape, niter);

    draw_shapes(&mut window, &shapes, &mut rs);
    while window.is_open() {
        for event in window.events() {
            match event {
                event::KeyPressed{code, ..} => {
                    match code {
                        Key::Escape => {
                            window.close();
                        }
                        Key::Q => {
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
fn main() {
    let args = (std::env::args().nth(1),
                if let Some(n) = std::env::args().nth(2) {
        n.parse().ok()
    } else {
        Some(8)
    });
    match args {
        (Some(rules), Some(niter)) => {
            match Grammar::from_bytes(rules.as_bytes()) {
                Ok(mut g) => process(&mut g, niter),
                Err(e) => println!("{:?}", e),
            }
        }
        (_, _) => println!("Usage: Grammar [N]\n"),

    }
}
