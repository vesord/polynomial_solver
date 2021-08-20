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

impl ops::AddAssign<Polynomial> for Polynomial {
    fn add_assign(&mut self, rhs: Polynomial) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
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

fn parse_term(term: &str) -> Result<Polynomial, String> {
    println!("Term: {} ", term);
    let mut poly = Polynomial { a: 0., b: 0., c: 0.};
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
        "X^0" => poly.c += coef,
        "" => poly.c += coef,
        "X^1" => poly.b += coef,
        "X" => poly.b += coef,
        "X^2" => poly.a += coef,
        "X*X" => poly.a += coef,
        _ => return Err("Bad pattern of term: ".to_owned() + term),
    }
    Ok(poly)
}

fn parse_equation(eq: &str) -> Result<Polynomial, String> {
    println!("Equation: {}", eq);
    if eq.is_empty() { return Err("Empty equation found".to_owned()); }
    let eq: String = String::from("+") + eq;
    let parse_term_cls = |term: &str| parse_term(term
            .chars()
            .take_while(|c| *c != '+' && *c != '-')
            .collect::<String>().as_str());
    let mut pos_term_res = Polynomial {a: 0., b: 0., c: 0.};
    let mut neg_term_res = Polynomial {a: 0., b: 0., c: 0.};
    for term in eq.split("+").map(parse_term_cls) {
        pos_term_res += term?;
    }
    for term in eq.split("-").map(parse_term_cls) {
        neg_term_res += term?;
    }
    Ok(pos_term_res - neg_term_res)
}

fn parse_polynomial(input: &str) -> Result<Polynomial, String> {
    let equations: Vec<&str> = input.split("=").collect();
    println!("{:?}", equations);
    match equations.len() {
        1 => Err("Symbol '=' not found".to_owned()),
        2 => Ok(parse_equation(equations[0])?
                - parse_equation(equations[1])?),
        _ => Err("More then one '='".to_owned()),
    }
}

fn main() {
    let input: String = String::from("-5 + 4 * X -+--+-X^2 = X^2");

    let input = input.remove_whitespaces().remove_redundant_operators();
    let poly: Polynomial = match parse_polynomial(&input) {
        Ok(poly) => poly,
        Err(err) => {
            println!("Parse error: {}", err);
            return;
        },
    };

    println!("{:?}", poly);
}
