mod polynomial;
use polynomial::{Polynomial, parse_polynomial};
use std::env;

fn print_help() {
    println!("Input polynomial as pattern: ");
    println!("a0 * X^0 + a1 * X^1 + a2 * X^2 + ... + an * X^n = b0 * X^0 + ... + bn * X^n");
    println!("Program can solve only polynomials with degree up to 2nd")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong input");
        print_help();
        return;
    }
    let input: String = args[1].to_string();
    let poly: Polynomial = match parse_polynomial(&input) {
        Ok(poly) => poly,
        Err(err) => {
            println!("Parse error: {}", err);
            print_help();
            return;
        },
    };
    poly.print_reduced();
    poly.print_degree();
    match poly.solve() {
        Ok(solution) => println!("Root(s): {}", solution),
        Err(err) => {
            println!("Solve error: {}", err);
            print_help();
            return;
        }
    }
}
