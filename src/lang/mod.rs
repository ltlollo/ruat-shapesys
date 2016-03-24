extern crate vec_map;
extern crate sfml;
extern crate itertools;

use self::vec_map::VecMap;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderStates};
use self::itertools::Itertools;

pub mod geom;
use self::geom::*;

fn is_vertex(c: u8) -> bool {
    match c {
        b'A'...b'Z' => true,
        _ => false,
    }
}
fn is_mid(c: u8) -> bool {
    match c {
        b'a'...b'z' => true,
        _ => false,
    }
}
fn is_center(c: u8) -> bool {
    c == b'.'
}
fn is_rhs_sepa(c: u8) -> bool {
    c == b','
}
fn is_rule_sepa(c: u8) -> bool {
    c == b'>'
}
fn is_legal(c: u8) -> bool {
    is_vertex(c) || is_mid(c) || is_center(c) || is_rhs_sepa(c) ||
    is_rule_sepa(c)
}

#[derive(Debug)]
pub enum RuleErr {
    UnknownSymbol {
        sym: u8,
    },
    PointSegmentation,
    EmptyCenter,
    NonVertexStart,
    NoSeparator,
    MultiSeparatos,
    NonUniqueRule,
}
#[derive(Debug)]
pub struct ParseErr {
    err: RuleErr,
    src: String,
}
#[derive(Clone)]
struct Rule {
    noadjmids_opt: bool,
    nocenter_opt: bool,
    gons: usize,
    self_cycle: usize,
    lhs: Vec<u8>,
    vrhs: Vec<Vec<u8>>,
    vmap: VecMap<Vector2f>,
    src: String,
}
impl Rule {
    pub fn vertices(&self) -> usize {
        self.gons
    }
    pub fn new<T: Into<String>>(rule: T) -> Result<Rule, ParseErr> {
        let src: String = rule.into();
        let (mid_opt, center_opt, gons, cycle, nlhs, rhsv) = {
            let rule = src.as_bytes();
            for &sym in rule.iter() {
                if !is_legal(sym) {
                    return Err(ParseErr {
                        err: RuleErr::UnknownSymbol { sym: sym },
                        src: src.clone(),
                    });
                }
            }
            let (mut lhs, mut rhs): (&[u8], &[u8]) = (&[], &[]);
            let (mut center_opt, mut mid_opt) = (true, true);
            let mut gons: usize = 0;
            let mut i = 0;
            for ele in rule.split(|c| is_rule_sepa(*c)) {
                if i == 0 {
                    lhs = ele;
                    gons = ele.iter().filter(|c| is_vertex(**c)).count();
                } else if i == 1 {
                    for &sym in ele.iter() {
                        if !lhs.iter()
                               .any(|&c| {
                                   c == sym || is_rhs_sepa(sym) ||
                                   is_center(sym)
                               }) {
                            return Err(ParseErr {
                                err: RuleErr::UnknownSymbol { sym: sym },
                                src: src.clone(),
                            });
                        }
                        if is_center(sym) {
                            center_opt = false;
                        }
                    }
                    rhs = ele;
                }
                i += 1;
            }
            if i < 2 {
                return Err(ParseErr {
                    err: RuleErr::NoSeparator,
                    src: src.clone(),
                });
            } else if i > 2 {
                return Err(ParseErr {
                    err: RuleErr::MultiSeparatos,
                    src: src.clone(),
                });
            }
            if lhs.len() > 1 {
                for j in 0..lhs.len() - 1 {
                    if is_mid(lhs[j]) && is_mid(lhs[j + 1]) {
                        mid_opt = false;
                    }
                }
            }
            if gons < 2 && lhs.len() != gons {
                return Err(ParseErr {
                    err: RuleErr::PointSegmentation,
                    src: src.clone(),
                });
            }
            if gons == 0 && !center_opt {
                return Err(ParseErr {
                    err: RuleErr::EmptyCenter,
                    src: src.clone(),
                });
            }
            let rhsv: Vec<Vec<u8>> = rhs.split(|c| is_rhs_sepa(*c))
                                        .filter(|seq| seq.len() > 0)
                                        .map(|seq| {
                                            seq.iter().cloned().collect()
                                        })
                                        .collect();
            let cycle = rhsv.iter()
                            .position(|ref v| {
                                v.len() == gons &&
                                v.iter().all(|&c| is_vertex(c))
                            })
                            .unwrap_or(rhsv.len());
            let mut nlhs: Vec<u8> = lhs.iter().cloned().collect();
            if !lhs.is_empty() {
                if !is_vertex(lhs[0]) {
                    return Err(ParseErr {
                        err: RuleErr::NonVertexStart,
                        src: src.clone(),
                    });
                } else {
                    nlhs.reserve_exact(1);
                    nlhs.push(lhs[0]);
                }
            }
            (mid_opt, center_opt, gons, cycle, nlhs, rhsv)
        };
        Ok(Rule {
            noadjmids_opt: mid_opt,
            nocenter_opt: center_opt,
            gons: gons,
            self_cycle: cycle,
            lhs: nlhs,
            vrhs: rhsv,
            vmap: VecMap::new(),
            src: src,
        })
    }
    fn calc_mids(&mut self) {
        if self.noadjmids_opt {
            for i in 0..self.lhs.len() {
                if is_mid(self.lhs[i]) {
                    let val = mid(&self.vmap[self.lhs[i - 1] as usize],
                                  &self.vmap[self.lhs[i + 1] as usize]);
                    self.vmap.insert(self.lhs[i] as usize, val);
                }
            }
        } else {
            let mut i = 0;
            while i < self.lhs.len() {
                let i_mb = i;
                while is_mid(self.lhs[i]) {
                    i += 1;
                }
                let n_mids = i - i_mb;
                for j in 0..n_mids {
                    let val = div(&self.vmap[self.lhs[i_mb - 1] as usize],
                                  &self.vmap[self.lhs[i] as usize],
                                  (j + 1) as f32,
                                  (n_mids + 1) as f32);
                    self.vmap.insert(self.lhs[i_mb + j] as usize, val);
                }
                i += 1;
            }
        }
    }
    pub fn apply(&mut self,
                 win: &mut RenderWindow,
                 rs: &mut RenderStates,
                 shape: &Shape)
                 -> Vec<Shape> {
        let mut res = Vec::with_capacity(self.vrhs.len());
        let mut i = 0;
        for ele in shape.iter() {
            while !is_vertex(self.lhs[i]) {
                i = i + 1;
            }
            self.vmap.insert(self.lhs[i] as usize, *ele);
            i = i + 1;
        }
        self.calc_mids();
        if !self.nocenter_opt {
            self.vmap.insert('.' as usize, shape.center());
        }
        for i in 0..self.vrhs.len() {
            let shape: Shape = self.vrhs[i]
                                   .iter()
                                   .map(|s| self.vmap[*s as usize])
                                   .collect();
            if i != self.self_cycle {
                res.push(shape);
            } else {
                shape.draw(win, rs);
            }
        }
        res
    }
}
impl<'a> Into<String> for &'a Rule {
    fn into(self) -> String {
        self.src.clone()
    }
}
impl<'a> Into<String> for &'a mut Grammar {
    fn into(self) -> String {
        let res: Vec<String> = self.pmap
                                   .iter()
                                   .map(|(_, s)| s.into())
                                   .collect();
        res.join(";")
    }
}
impl<'a> Into<String> for &'a Grammar {
    fn into(self) -> String {
        let res: Vec<String> = self.pmap
                                   .iter()
                                   .map(|(_, s)| s.into())
                                   .collect();
        res.join(";")
    }
}

pub struct Grammar {
    pmap: VecMap<Rule>,
}
impl Grammar {
    fn from_rules(rules: &[Rule]) -> Result<Grammar, ParseErr> {
        for (i, j) in rules.iter().combinations() {
            if i.vertices() == j.vertices() {
                return Err(ParseErr {
                    err: RuleErr::NonUniqueRule,
                    src: j.into(),
                });
            }
        }
        Ok(Grammar {
            pmap: rules.iter().map(|r| (r.vertices(), r.clone())).collect(),
        })
    }
    pub fn new<T: Into<String>>(rules: T) -> Result<Grammar, ParseErr> {
        let res: Result<Vec<_>, ParseErr> = rules.into()
                                                 .split(|s| s == ';')
                                                 .map(|s| Rule::new(s))
                                                 .collect();
        match res {
            Ok(rules) => Grammar::from_rules(&rules[..]),
            Err(err) => Err(err),
        }
    }
    pub fn apply(&mut self,
                 win: &mut RenderWindow,
                 rs: &mut RenderStates,
                 shape: &Shape)
                 -> Vec<Shape> {
        match self.pmap.get_mut(shape.gons()) {
            Some(r) => r.apply(win, rs, &shape),
            None => vec![shape.clone()],
        }
    }
    pub fn next(&mut self,
                win: &mut RenderWindow,
                rs: &mut RenderStates,
                state: &Vec<Shape>)
                -> Vec<Shape> {
        state.iter().fold(Vec::new(), |mut res, shape| {
            res.extend(self.apply(win, rs, shape));
            res
        })
    }
    pub fn iterate(&mut self,
                   win: &mut RenderWindow,
                   rs: &mut RenderStates,
                   state: &Vec<Shape>,
                   depth: u8)
                   -> Vec<Shape> {
        (0..depth).fold(state.clone(), |state, _| self.next(win, rs, &state))
    }
}
