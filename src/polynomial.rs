use std::collections::HashMap;
use std::ops;

mod f64_utils;
use f64_utils::F64Utils;
mod string_utils;
mod polynomial_test;

use string_utils::StringUtils;

#[derive(Debug)]
pub struct Polynomial {
    pub degrees: HashMap<i32, f64>,
}

impl Polynomial {
    pub fn print_reduced(&self) {
        let mut m: Vec<(&i32, &f64)> = self.degrees.iter().collect();
        m.sort_by(|a, b| b.0.cmp(a.0));
        let reduced_terms: Vec<String> = m.iter().map(|kv| format!("{}*X^{}", kv.1, kv.0)).collect();
        print!("Reduced form: ");
        println!("{} = 0", reduced_terms.join(" + "));
    }

    pub fn print_degree(&self) {
        let d = self.get_degree();
        println!("Degree: {}", d);
    }

    pub fn solve(&self) -> Result<String, String> {
        match self.get_degree() {
            0 => Ok(self.solve_zero_degree()),
            1 => Ok(self.solve_first_degree()),
            2 => Ok(self.solve_second_degree()),
            _ => Err("Can be solved only 0, 1, 2 degree polynomials".to_owned()),
        }
    }

    fn get_square_coefs(&self) -> (f64, f64, f64) {
        let a = *self.degrees.get(&2).unwrap_or(&0.);
        let b = *self.degrees.get(&1).unwrap_or(&0.);
        let c = *self.degrees.get(&0).unwrap_or(&0.);
        (a, b, c)
    }

    fn get_discriminant(&self) -> f64 {
        let (a, b, c) = self.get_square_coefs();
        b * b - 4. * a * c
    }

    fn get_degree(&self) -> i32 {
        let mut max_degree = 0;
        for (degree, value) in &self.degrees {
            if *value != 0. && *degree > max_degree {
                max_degree = *degree;
            }
        }
        max_degree
    }

    fn solve_zero_degree(&self) -> String {
        println!("Solving zero-degree polynomial");
        let (_, _, c) = self.get_square_coefs();
        if c == 0. { return "No solution, polynomial is correct".to_owned() }
        "No solution, polynomial is incorrect".to_owned()
    }

    fn solve_first_degree(&self) -> String {
        println!("Solving first-degree polynomial");
        let (_, b, c) = self.get_square_coefs();
        let res = -c / b;
        res.positivize_zero().to_string()
    }

    fn solve_second_degree(&self) -> String {
        println!("Solving second-degree polynomial");
        let d: f64 = self.get_discriminant();
        println!("Discriminant: {}", d);
        match d {
            d if d < 0. => self.solve_second_degree_neg_d(d),
            d if d > 0. => self.solve_second_degree_pos_d(d),
            _ => self.solve_second_degree_zero_d(),
        }
    }

    fn solve_second_degree_neg_d(&self, d: f64) -> String {
        println!("Negative discriminant. Finding complex roots...");
        let (a, b, _) = self.get_square_coefs();
        let x1: (f64, f64) = ((-b / 2. / a).positivize_zero(), ((-d).ft_sqrt() / 2. / a).positivize_zero());
        let x2: (f64, f64) = ((-b / 2. / a).positivize_zero(), ((-d).ft_sqrt() / 2. / a).positivize_zero());
        format!("{}+{}i,{}-{}i", x1.0, x1.1, x2.0, x2.1)
    }

    fn solve_second_degree_pos_d(&self, d: f64) -> String {
        println!("Positive discriminant. Finding real roots...");
        let (a, b, _) = self.get_square_coefs();
        let x1: f64 = (-b + d.ft_sqrt()) / 2. / a;
        let x2: f64 = (-b - d.ft_sqrt()) / 2. / a;
        x1.to_string() + "," + x2.to_string().as_str()
    }

    fn solve_second_degree_zero_d(&self) -> String {
        println!("Zero discriminant. Finding real roots...");
        let (a, b, _) = self.get_square_coefs();
        let x: f64 = -b / 2. / a;
        x.positivize_zero().to_string() + "," + x.positivize_zero().to_string().as_str()
    }
}

impl ops::AddAssign<Polynomial> for Polynomial {
    fn add_assign(&mut self, rhs: Polynomial) {
        for (degree, value) in &rhs.degrees {
            let deg = self.degrees.entry(*degree).or_insert(0.);
            *deg += value;
        }
    }
}

impl ops::Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Self::Output {
        let mut res= self;
        for (degree, value) in &rhs.degrees {
            let deg = res.degrees.entry(*degree).or_insert(0.);
            *deg -= value;
        }
        res
    }
}

fn parse_degree(degree: &String) -> Result<i32, String> {
    if !degree.starts_with("X^") { return Err("Bad pattern of term: ".to_owned() + degree.as_str()) }
    match degree.chars().skip(2).collect::<String>().parse::<i32>() {
        Ok(deg) => Ok(deg),
        Err(err) => Err(err.to_string()),
    }
}

fn parse_f64(s: &str) -> (f64, usize) {
    let mut parsed: usize = 0;
    let mut was_dot: bool = false;
    for ch in s.chars() {
        if !ch.is_numeric() {
            if !was_dot && ch == '.' {
                was_dot = true;
            }
            else {
                break;
            }
        }
        parsed += 1;
    }
    let f = s.chars().take(parsed).collect::<String>().parse::<f64>().unwrap_or(1.);
    (f, parsed)
}

fn parse_term(term: &str) -> Result<Polynomial, String> {
    let mut poly = Polynomial { degrees: HashMap::new() };
    if term.is_empty() { return Ok(poly); }
    if term.starts_with("*") || !term.ends_with(|c: char| c.is_numeric() || c == 'X') { return Err("Bad pattern of term: ".to_owned() + term); }

    let (coef, parsed) = parse_f64(term);

    let mut remain = term.chars().skip(parsed).collect::<String>();
    if remain.starts_with("*") { remain.remove(0); }
    match remain.as_str() {
        "" => poly.degrees.insert(0, coef),
        "X" => poly.degrees.insert(1, coef),
        _ => poly.degrees.insert(parse_degree(&remain)?, coef),
    };
    Ok(poly)
}

fn parse_equation(eq: &str) -> Result<Polynomial, String> {
    if eq.is_empty() { return Err("Empty equation found".to_owned()); }
    let eq: String = String::from("+") + eq;
    let parse_term_cls = |term: &str| parse_term(term
        .chars()
        .take_while(|c| *c != '+' && *c != '-')
        .collect::<String>().as_str());
    let mut pos_term_res = Polynomial { degrees: HashMap::new() };
    let mut neg_term_res = Polynomial { degrees: HashMap::new() };
    for term in eq.split("+").map(parse_term_cls) {
        pos_term_res += term?;
    }
    for term in eq.split("-").map(parse_term_cls) {
        neg_term_res += term?;
    }
    Ok(pos_term_res - neg_term_res)
}

pub fn parse_polynomial(input: &String) -> Result<Polynomial, String> {
    let input = input.remove_whitespaces().remove_redundant_operators();
    let equations: Vec<&str> = input.split("=").collect();
    match equations.len() {
        1 => Err("Symbol '=' not found".to_owned()),
        2 => Ok(parse_equation(equations[0])?
            - parse_equation(equations[1])?),
        _ => Err("More then one '='".to_owned()),
    }
}
