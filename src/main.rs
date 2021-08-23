mod polynomial;
use polynomial::{Polynomial, parse_polynomial};

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
