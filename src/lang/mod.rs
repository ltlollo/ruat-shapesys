#![allow(dead_code)]

extern crate vec_map;

use self::vec_map::VecMap;
use sfml::system::Vector2f;

pub mod geom;
use self::geom::*;

use std::string::String;

pub fn is_vertex(c: u8) -> bool {
    const A: u8 = 'A' as u8;
    const Z: u8 = 'Z' as u8;
    match c {
        A...Z => true,
        _ => false,
    }
}

pub fn is_mid(c: u8) -> bool {
    const A: u8 = 'a' as u8;
    const Z: u8 = 'z' as u8;
    match c {
        A...Z => true,
        _ => false,
    }
}

pub fn is_center(c: u8) -> bool {
    c == '.' as u8
}
pub fn is_rhs_sepa(c: u8) -> bool {
    c == ',' as u8
}
pub fn is_rule_sepa(c: u8) -> bool {
    c == '>' as u8
}

pub fn is_legal(c: u8) -> bool {
    is_vertex(c) || is_mid(c) || is_center(c) || is_rhs_sepa(c) ||
    is_rule_sepa(c)
}

#[derive(Debug)]
pub enum RuleErr {
    UnknownSymbol {
        s: u8,
    },
    PointSegmentation,
    EmptyCenter,
    NonVertexStart,
    NoSeparator,
    MultiSeparatos,
}

pub struct Rule {
    no_adjacent_mids_opt: bool,
    no_center_opt: bool,
    n_gons: usize,
    lhs: Vec<u8>,
    vrhs: Vec<Vec<u8>>,
    vmap: VecMap<Vector2f>,
}

impl Rule {
    pub fn clone(&self) -> Rule {
        Rule {
            no_adjacent_mids_opt: self.no_adjacent_mids_opt,
            no_center_opt: self.no_center_opt,
            n_gons: self.n_gons,
            lhs: self.lhs.clone(),
            vrhs: self.vrhs.clone(),
            vmap: self.vmap.clone(),
        }
    }
    pub fn from_bytes(rule: &[u8]) -> Result<Rule, RuleErr> {
        for &sym in rule.iter() {
            if !is_legal(sym) {
                return Err(RuleErr::UnknownSymbol { s: sym });
            }
        }
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
                        return Err(RuleErr::UnknownSymbol { s: sym });
                    }
                    if is_center(sym) {
                        no_center_opt = false;
                    }
                }
                rhs = ele;
            }
            i = i + 1;
        }
        if i < 2 {
            return Err(RuleErr::NoSeparator);
        } else if i > 2 {
            return Err(RuleErr::MultiSeparatos);
        }
        if lhs.len() > 1 {
            for j in 0..lhs.len() - 1 {
                if is_mid(lhs[j]) && is_mid(lhs[j + 1]) {
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
                                    .map(|seq| seq.iter().cloned().collect())
                                    .collect();
        let mut nlhs: Vec<u8> = lhs.iter().cloned().collect();
        if !lhs.is_empty() {
            if !is_vertex(lhs[0]) {
                return Err(RuleErr::NonVertexStart);
            } else {
                nlhs.reserve_exact(1);
                nlhs.push(lhs[0]);
            }
        }
        Ok(Rule {
            no_adjacent_mids_opt: no_adjacent_mids_opt,
            no_center_opt: no_center_opt,
            n_gons: n_gons,
            lhs: nlhs,
            vrhs: rhsv,
            vmap: VecMap::new(),
        })
    }
    pub fn from_vec(v: &Vec<u8>) -> Result<Rule, RuleErr> {
        Rule::from_bytes(&v[..])
    }
    pub fn calc_mids(&mut self) {
        if self.no_adjacent_mids_opt {
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
                    i = i + 1;
                }
                let n_mids = i - i_mb;
                for j in 0..n_mids {
                    let val = div_seg(&self.vmap[self.lhs[i_mb - 1] as usize],
                                      &self.vmap[self.lhs[i] as usize],
                                      (j + 1) as f32,
                                      (n_mids + 1) as f32);
                    self.vmap.insert(self.lhs[i_mb + j] as usize, val);
                }
                i = i + 1;
            }
        }
    }
    pub fn apply(&mut self, shape: &Shape) -> Vec<Shape> {
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
        if !self.no_center_opt {
            self.vmap.insert('.' as usize, calc_center(shape));
        }
        for rhs in self.vrhs.iter() {
            res.push(rhs.iter().map(|s| self.vmap[*s as usize]).collect())
        }
        res
    }
    pub fn as_string(&self) -> String {
        let mut res = self.lhs.clone();
        let sepa = ',' as u8;
        let mut rhs = self.vrhs[..].join(&sepa);
        res.push('>' as u8);
        res.append(&mut rhs);
        String::from_utf8(res).unwrap()
    }
}

#[derive(Debug)]
pub enum GrammarErr {
    NonUniqueRule,
}

pub struct Grammar {
    pmap: VecMap<Rule>,
}

impl Grammar {
    pub fn new(rules: &[Rule]) -> Result<Grammar, GrammarErr> {
        if rules.len() > 1 {
            for i in 0..rules.len() - 1 {
                for j in i + 1..rules.len() {
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
        Ok(Grammar { pmap: pmap })
    }
    pub fn apply_rule(&mut self, shape: &Shape) -> Vec<Shape> {
        match self.pmap.get_mut(shape.len()) {
            Some(r) => r.apply(&shape),
            None => vec![shape.clone()],
        }
    }
    pub fn next(&mut self, state: &Vec<Shape>) -> Vec<Shape> {
        state.iter().fold(Vec::new(), |mut res, shape| {
            res.extend(self.apply_rule(shape));
            res
        })
    }
    pub fn iterate(&mut self, state: &Vec<Shape>, depth: u8) -> Vec<Shape> {
        (0..depth).fold(state.clone(), |state, _| self.next(&state))
    }
    pub fn as_string(&self) -> String {
        let res: Vec<String> = self.pmap
                                   .iter()
                                   .map(|(_, s)| s.as_string())
                                   .collect();
        res.join(";")
    }
}

// grammar explanation
// def: RULE := LHS '>' RHS
//      LHS  := [A-Z][:alpha:]
//      RHS  := "" | [:alpha:] | "." | RHS ',' RHS
// ex: AbCdEf>ACE,bdf
//      - AbCdEf>_ instructs the parser to match an ACE shaped plygon,
//        introducing b,d,f points between it's vertices
//      - _>aBc instucts the parser to form a new aBc polygon with using the
//        vertices introduced in LHS
//      - Old vertices must be uppercase, new ones lowercase.
//      - The LHS definition wraps arownd, therfore in "ABCd", d is considered
//        between A and C (*)
//      - '.' introduces the center of the polygon
// def: RULES := RULE | RULE, RULES
//      - rules LHS must match unique polygons
//        ( ex: "ABC>", "AdBC>" is not allowed )
//
