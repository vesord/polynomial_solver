use std::collections::HashMap;

mod polynomial;
use polynomial::Polynomial;
mod string_utils;
use string_utils::StringUtils;

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
    poly.print_degree();
    match poly.solve() {
        Ok(solution) => println!("Root(s): {}", solution),
        Err(err) => {
            println!("Solve error: {}", err);
            return;
        }
    }
}
