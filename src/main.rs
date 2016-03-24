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

fn poly(src: String) -> Option<Shape> {
    src.split(";")
       .map(|point| {
           let (mut i, mut v) = (0, Vector2f { x: 0f32, y: 0f32 });
           for coord in point.split(",") {
               match i {
                   0 => {
                       if let Ok(x) = coord.parse() {
                           v.x = x;
                       } else {
                           return None;
                       }
                   }
                   1 => {
                       if let Ok(y) = coord.parse() {
                           v.y = y;
                       } else {
                           return None;
                       }
                   }
                   _ => return None,
               }
               i = i + 1;
           }
           Some(v)
       })
       .collect()
}

fn process(g: &mut Grammar, n: u8, state: Vec<Shape>) {
    let mut window = RenderWindow::new(VideoMode::new_init(WIDTH, HEIGHT, 32),
                                       "shapesys",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
                         .expect("Cannot create a Render Window.");

    window.clear(&Color::black());
    window.display();
    let mut rs = RenderStates::default();
    let shapes = g.iterate(&mut window, &mut rs, &state, n);

    shapes.draw(&mut window, &mut rs);
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
                                let gram: String = g.into();
                                img.save_to_file(&(gram + ".png"));
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
fn help() {
    println!("Usage: Grammar [N] [POLY]");
}
fn main() {
    let shape = vec![Vector2f {
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
                     }]
                    .into();
    let args = (std::env::args().nth(1),
                if let Some(n) = std::env::args().nth(2) {
        n.parse().ok()
    } else {
        Some(8)
    },
                if let Some(s) = std::env::args().nth(3) {
        poly(s)
    } else {
        Some(shape)
    });
    match args {
        (None, _, _) => {
            println!("Error: missing grammar");
            help();
        }
        (Some(rules), Some(n), Some(shape)) => {
            match Grammar::new(rules) {
                Ok(mut g) => process(&mut g, n, vec![shape]),
                Err(e) => {
                    println!("Error: parsing grammar\nWhat: {:?}", e);
                    help();
                }
            }
        }
        (_, None, _) => {
            println!("Error: parsing N");
            help();
        }
        (_, _, None) => {
            println!("Error: parsing POLY");
            help();
        }

    }
}
