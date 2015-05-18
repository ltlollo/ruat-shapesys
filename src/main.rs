#![feature(collections)]

extern crate sfml;

const WIDTH : u32 = 1020;
const HEIGHT: u32 = 1020;
const OFF   : f32 = 10.0;

use std::collections::VecMap;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{RenderWindow, RenderTarget, Color, Vertex, PrimitiveType};

type Shape = Vec<Vertex>;

fn mid(f: &Vertex, s: &Vertex) -> Vertex {
    Vertex::new_with_pos(&Vector2f{
        x: (f.position.x + s.position.x)/2f32,
        y: (f.position.y + s.position.y)/2f32
    })
}

fn div_seg(f: &Vertex, s: &Vertex, of: f32, n: f32) -> Vertex {
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

fn draw_shapes(window: &mut RenderWindow, shapes: &Vec<Shape>) {
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

fn calc_center(shape: &Shape) -> Vertex {
    let mut c = Vector2f::new(0f32, 0f32);
    for v in shape.iter() {
        c.x = c.x + v.position.x;
        c.y = c.y + v.position.y;
    }
    c.x = c.x/shape.len() as f32;
    c.y = c.y/shape.len() as f32;
    Vertex::new_with_pos(&c)
}

fn is_vertex(c: u8) -> bool {
    const A: u8 = 'A' as u8;
    const Z: u8 = 'Z' as u8;
    match c {
        A...Z => true,
        _     => false,
    }
}

fn is_mid(c: u8) -> bool {
    const A: u8 = 'a' as u8;
    const Z: u8 = 'z' as u8;
    match c {
        A...Z => true,
        _     => false,
    }
}

fn is_center(c: u8) -> bool { c == '.' as u8 }
fn is_rhs_sepa(c: u8) -> bool { c == ',' as u8 }
fn is_rule_sepa(c: u8) -> bool { c == '>' as u8 }

fn is_legal(c: u8) -> bool {
    is_vertex(c) || is_mid(c) || is_center(c) || is_rhs_sepa(c) ||
        is_rule_sepa(c)
}

fn del_garbage(symbols: &mut Vec<u8>) {
   symbols.retain(|c| is_legal(*c));
}

#[derive(Debug)]
enum RuleErr {
    UnknownSymbol{ s: u8},
    PointSegmentation,
    EmptyCenter,
    NonVertexStart,
    NoSeparator,
    MultiSeparatos,
}

struct Rule {
    no_adjacent_mids_opt: bool,
    no_center_opt: bool,
    n_gons: usize,
    lhs: Vec<u8>,
    vrhs: Vec<Vec<u8>>,
    vmap: VecMap<Vertex>,
}

impl Rule {
    fn clone(&self) -> Rule {
        Rule{
            no_adjacent_mids_opt: self.no_adjacent_mids_opt,
            no_center_opt: self.no_center_opt,
            n_gons: self.n_gons,
            lhs: self.lhs.clone(),
            vrhs: self.vrhs.clone(),
            vmap: self.vmap.clone(),
        }
    }
    fn from_bytes(rule: &[u8]) -> Result<Rule, RuleErr> {
        let mut lhs: &[u8] = &[];
        let mut rhs: &[u8] = &[];
        let mut no_center_opt = true;
        let mut no_adjacent_mids_opt = true;
        let mut n_gons: usize = 0;

        let mut i = 0;
        for ele in rule.split(|c| is_rule_sepa(*c)) {
            if i == 0 {
                lhs = ele;
                n_gons = ele.iter().filter(|c| is_vertex(**c)).count();
            } else if i == 1 {
                for &sym in ele.iter() {
                   if !lhs.iter()
                          .any(|&c| {
                              c == sym || is_rhs_sepa(sym) || is_center(sym)
                          }) {
                              return Err(RuleErr::UnknownSymbol{ s: sym });
                   }
                   if is_center(sym) {
                        no_center_opt = false;
                   }
                }
                rhs = ele;
            }
            i  = i+1;
        }
        if i < 2 {
            return Err(RuleErr::NoSeparator);
        } else if i > 2 {
            println!("i {}", i);
            return Err(RuleErr::MultiSeparatos);
        }
        if lhs.len() > 1 {
            for j in (0..lhs.len()-1) {
                if is_mid(lhs[j]) && is_mid(lhs[j+1]) {
                    no_adjacent_mids_opt = false;
                }
            }
        }
        if n_gons < 2 && lhs.len() != n_gons {
            return Err(RuleErr::PointSegmentation);
        }
        if n_gons == 0 && !no_center_opt {
            return Err(RuleErr::EmptyCenter);
        }
        let rhsv: Vec<Vec<u8>> = rhs.split(|c| is_rhs_sepa(*c))
                                    .filter(|seq| seq.len() > 0)
                                    .map(|seq| {
                                        seq.iter().map(|c| *c).collect()
                                    })
                                    .collect();
        let mut nlhs: Vec<u8> = lhs.iter().map(|a| *a).collect();
        if !lhs.is_empty() {
            if !is_vertex(lhs[0]) {
                return Err(RuleErr::NonVertexStart);
            } else {
                nlhs.reserve_exact(1);
                nlhs.push(lhs[0]);
            }
        }
        Ok(Rule{
            no_adjacent_mids_opt: no_adjacent_mids_opt,
            no_center_opt: no_center_opt,
            n_gons: n_gons,
            lhs: nlhs,
            vrhs: rhsv,
            vmap: VecMap::new(),
        })
    }
    fn calc_mids(&mut self) {
        if self.no_adjacent_mids_opt {
            for i in (0..self.lhs.len()) {
                if is_mid(self.lhs[i]) {
                    let val = mid(&self.vmap[self.lhs[i-1] as usize],
                                  &self.vmap[self.lhs[i+1] as usize]);
                    self.vmap.insert(self.lhs[i] as usize, val);
                }
            }
        } else {
            for mut i in (0..self.lhs.len()) {
                let i_mb = i;
                while is_mid(self.lhs[i]) {
                    i = i+1;
                }
                let n_mids = i - i_mb;
                for j in (0..n_mids) {
                    let val = div_seg(&self.vmap[self.lhs[i_mb-1] as usize],
                                      &self.vmap[self.lhs[i] as usize],
                                      (j+1) as f32, (n_mids+1) as f32);
                    self.vmap.insert(self.lhs[i_mb+j] as usize, val);
                }
            }
        }
    }
    fn apply(&mut self, shape: &Shape) -> Vec<Shape> {
        let mut res = Vec::with_capacity(self.vrhs.len());
        let mut i = 0;
        for ele in shape.iter() {
            while !is_vertex(self.lhs[i]) {
                i = i+1;
            }
            self.vmap.insert(self.lhs[i] as usize, *ele);
            i = i+1;
        }
        self.calc_mids();
        if !self.no_center_opt {
            self.vmap.insert('.' as usize, calc_center(shape));
        }
        for rhs in self.vrhs.iter() {
            res.push(rhs.iter().map(|s| self.vmap[*s as usize]).collect())
        }
        res
    }
}

#[derive(Debug)]
enum GrammarErr {
    NonUniqueRule,
}

struct Grammar {
    pmap: VecMap<Rule>,
}

impl Grammar {
    fn new(rules: &[Rule]) -> Result<Grammar, GrammarErr> {
        if rules.len() > 1 {
            for i in (0..rules.len()-1) {
                for j in (i+1..rules.len()) {
                    if rules[i].n_gons == rules[j].n_gons {
                        return Err(GrammarErr::NonUniqueRule);
                    }
                }
            }
        }
        let mut pmap = VecMap::with_capacity(rules.len());
        for rule in rules.iter() {
            pmap.insert(rule.n_gons, rule.clone());
        }
        Ok(Grammar{
            pmap: pmap,
        })
    }
    fn apply_rule(&mut self, shape: &Shape) -> Vec<Shape> {
        match self.pmap.get_mut(&shape.len()) {
            Some(r) => r.apply(&shape),
            None    => vec![shape.clone()],
        }
    }
    fn next(&mut self, state: &Vec<Shape>) -> Vec<Shape> {
        let mut res = Vec::new();
        for shape in state.iter() {
            res.push_all(&self.apply_rule(shape)[..]);
        }
        res
    }
    fn iterate(&mut self, state: &Vec<Shape>, depth: u8) -> Vec<Shape> {
        let mut res = state.clone();
        for _ in (0..depth) {
            res = self.next(&res);
        }
        res
    }
}

/* grammar explanation
 * def: RULE := LHS '>' RHS
 *      LHS  := [A-Z][:alpha:]
 *      RHS  := "" | [:alpha:] | "." | RHS ',' RHS
 * ex: AbCdEf>ACE,bdf
 *      - AbCdEf>_ instructs the parser to match an ACE shaped plygon,
 *        introducing b,d,f points between it's vertices
 *      - _>aBc instucts the parser to form a new aBc polygon with using the
 *        vertices introduced in LHS
 *      - Old vertices must be uppercase, new ones lowercase.
 *      - The LHS definition wraps arownd, therfore in "ABCd", d is considered
 *        between A and C (*)
 *      - '.' introduces the center of the polygon
 * def: RULES := RULE | RULE, RULES
 *      - rules LHS must match unique polygons
 *        ( ex: "ABC>", "AdBC>" is not allowed )
 */
fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(WIDTH, HEIGHT, 32),
                                       "shapesys",
                                       Close,
                                       &ContextSettings::default())
                                    .expect("Cannot create a Render Window.");

    window.clear(&Color::black());
    window.display();

    let f = Rule::from_bytes(
        b"AannBbnnCcnnDdnn>aBb,bCc,cDd,dAa,abcd"
        ).unwrap();
    let s = Rule::from_bytes(
        b"AavBbvCcv>cAa,aBb,bCc"
        ).unwrap();

    let mut g = Grammar::new(&[f, s]).unwrap();

    let first_shape: Vec<Shape> = vec![
        vec![
            Vertex::new_with_pos(&Vector2f{ x: 0f32   +OFF, y: 0f32   +OFF}),
            Vertex::new_with_pos(&Vector2f{ x: 0f32   +OFF, y: 1000f32+OFF}),
            Vertex::new_with_pos(&Vector2f{ x: 1000f32+OFF, y: 1000f32+OFF}),
            Vertex::new_with_pos(&Vector2f{ x: 1000f32+OFF, y: 0f32   +OFF})
        ]
    ];
    let shapes = g.iterate(&first_shape, 10);
    draw_shapes(&mut window, &shapes);
    while window.is_open() {
        for event in window.events() {
            match event {
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => window.close(),
                    _  => (),
                },
                event::Closed => window.close(),
                _             => (),
            }
        }
        window.display();
    }
}
