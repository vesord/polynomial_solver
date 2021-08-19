use std::ops;
use std::iter;

#[derive(Debug)]
struct Polynomial {
    a: f64,
    b: f64,
    c: f64,
}

impl iter::Sum<Polynomial> for Polynomial {
    fn sum<I: Iterator<Item=Polynomial>>(iter: I) -> Self {
        let mut res = Polynomial {a: 0., b: 0., c: 0.};
        for i in iter {
            res.a += i.a;
            res.b += i.b;
            res.c += i.c;
        };
        Polynomial {a: res.a, b: res.b, c: res.c}
    }
}

impl ops::Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}

impl ops::Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
        }
    }
}

fn parse_term(term: &str) -> Result<Polynomial, &str> {
    println!("Term: {}", term);
    let mut poly = Polynomial { a: 0., b: 0., c: 0.};

    Ok(poly)
}

fn parse_equation(eq: &str) -> Result<Polynomial, &str> {
    println!("Equation: {}", eq);
    if eq.is_empty() { return Err("Empty equation found"); }
    let eq: String = String::from("+") + eq;
    let parse_term_cls =  |term: &str| parse_term(term
        .chars()
        .take_while(|c| *c != '+' && *c != '-')
        .collect::<String>().as_str())
        .unwrap();
    let pos_terms: Polynomial = eq.split("+").map(parse_term_cls).sum();
    let neg_terms: Polynomial = eq.split("-").map(parse_term_cls).sum();
    Ok(pos_terms - neg_terms)
}

fn parse_polynomial(input: &str) -> Result<Polynomial, &str> {
    let equations: Vec<&str> = input.split("=").collect();
    println!("{:?}", equations);
    match equations.len() {
        1 => Err("Symbol '=' not found"),
        2 => Ok(parse_equation(equations[0].trim()).unwrap()),
        _ => Err("More then one '='"),
    }
}

fn main() {
    let input: String = String::from("5 + 4 * X + X^2 = X^2");

    let input_no_spaces = input.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let poly: Polynomial = parse_polynomial(&input_no_spaces).unwrap();

    println!("{:?}", poly);
}
