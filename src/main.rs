use std::ops;
use std::collections::HashMap;

#[derive(Debug)]
struct Polynomial {
    degrees: HashMap<i32, f64>,
}

impl Polynomial {
    fn print_reduced(&self) {
        let mut m: Vec<(&i32, &f64)> = self.degrees.iter().collect();
        m.sort_by(|a, b| b.0.cmp(a.0));
        let reduced_terms: Vec<String> = m.iter().map(|kv| format!("{}*X^{}", kv.1, kv.0)).collect();
        print!("Reduced form: ");
        println!("{} = 0", reduced_terms.join(" + "));
    }

    fn print_degree(&self) {
        let d = self.get_degree();
        println!("Degree: {}", d);
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
        (-c / b).to_string()
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
        "neg d".to_owned()
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
        "zero d".to_owned()
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

trait StringUtils {
    fn remove_whitespaces(&self) -> Self;
    fn remove_redundant_operators(&self) -> Self;
}

impl StringUtils for String {
    fn remove_whitespaces(&self) -> Self {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }

    fn remove_redundant_operators(&self) -> Self {
        let mut str = String::from(self);
        while str.contains("--") || str.contains("-+") || str.contains("+-") || str.contains("++") {
            str = str
                .replace("--", "+")
                .replace("++", "+")
                .replace("-+", "-")
                .replace("+-", "-");
        }
        str
    }
}

trait F64Utils {
    fn ft_sqrt(self) -> f64;
}

impl F64Utils for f64 {
    fn ft_sqrt(self) -> f64 {
        if self <= 0. { return self }
        let mut sqrt = self / 2.;
        let mut tmp = 0.;
        while sqrt != tmp {
            tmp = sqrt;
            sqrt = (self / tmp + tmp) / 2.;
        }
        sqrt
    }
}

fn parse_degree(degree: &String) -> Result<i32, String> {
    if !degree.starts_with("X^") { return Err("Bad pattern of term: ".to_owned() + degree.as_str()) }
    match degree.chars().skip(2).collect::<String>().parse::<i32>() {
        Ok(deg) => Ok(deg),
        Err(err) => Err(err.to_string()),
    }
}

fn parse_term(term: &str) -> Result<Polynomial, String> {
    // print!("Term: {} ", term);
    let mut poly = Polynomial { degrees: HashMap::new() };
    if term.is_empty() { return Ok(poly); }
    if term.starts_with("*") { return Err("Bad pattern of term".to_owned() + term); }

    let coef = term.chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()
        .unwrap_or(1.);

    let mut remain = term.chars().skip_while(|c| c.is_numeric()).collect::<String>();
    if remain.starts_with("*") { remain.remove(0); }
    match remain.as_str() {
        "" => poly.degrees.insert(0, coef),
        "X" => poly.degrees.insert(1, coef),
        _ => poly.degrees.insert(parse_degree(&remain)?, coef),
    };
    // println!("Poly: {:?} ", poly);
    Ok(poly)
}

fn parse_equation(eq: &str) -> Result<Polynomial, String> {
    // println!("Equation: {}", eq);
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
    // println!("Pos terms: {:?}", pos_term_res);
    // println!("Neg terms: {:?}", neg_term_res);
    Ok(pos_term_res - neg_term_res)
}

fn parse_polynomial(input: &String) -> Result<Polynomial, String> {
    println!("Input: {}", input);
    let input = input.remove_whitespaces().remove_redundant_operators();
    let equations: Vec<&str> = input.split("=").collect();
    // println!("{:?}", equations);
    match equations.len() {
        1 => Err("Symbol '=' not found".to_owned()),
        2 => Ok(parse_equation(equations[0])?
                - parse_equation(equations[1])?),
        _ => Err("More then one '='".to_owned()),
    }
}

fn solve_polynomial(poly: &Polynomial) -> Result<String, String> {
    poly.print_degree();
    match poly.get_degree() {
        0 => Ok(poly.solve_zero_degree()),
        1 => Ok(poly.solve_first_degree()),
        2 => Ok(poly.solve_second_degree()),
        _ => Err("Can be solved only 0, 1, 2 degree polynomials".to_owned()),
    }
}

fn main() {
    let input: String = String::from("-5 + 4 * X -X^2 = 14 * X^2 - 7X - 5");

    let poly: Polynomial = match parse_polynomial(&input) {
        Ok(poly) => poly,
        Err(err) => {
            println!("Parse error: {}", err);
            return;
        },
    };
    poly.print_reduced();
    match solve_polynomial(&poly) {
        Ok(solution) => println!("Solution: {}", solution),
        Err(err) => {
            println!("Solve error: {}", err);
            return;
        }
    }
}
