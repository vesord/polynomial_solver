#[cfg(test)]

use super::*;
use std::iter::FromIterator;
use std::array::IntoIter;
use float_eq;
use float_eq::FloatEqAll;

impl cmp::PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.degrees.eq_rmax_all(&other.degrees, &(f64::EPSILON * 10.))
    }
}

static POLY_PARSE_ZERO_DEGREE_CORRECT_TESTS: &'static [(&str, (i32, f64))] = &[
    (("7 = 7"), (0, 0.)),
    (("6 = 7"), (0, -1.)),
    (("7 = 6"), (0, 1.)),
    (("0 = 0"), (0, 0.)),
    (("0.000005 = 0.000007"), (0, -0.000_002)),
    (("0.05 + 0.5 - 12 + 79 = 0.07 + 22 - 0.1"), (0, 45.58)),
    (("-0.05 + 0.5 - 12 + 79 = -0.07 + 22 - 0.1"), (0, 45.62)),
    (("-0.05*X^0 + 0.5*X^0 - 12 + 79 = -0.07*X^0 + 22 - 0.1*X^0"), (0, 45.62)),
    (("-0.05*X^0 + 0.5*X^0 - 12 + 79 = -0.07*X^0 + 22 - 0.1*X^0"), (0, 45.62)),
    (("-0.0000000000001 = 0.0000000000002"), (0, -0.000_000_000_000_3)),
    (("-0.0000000000002 * X^0 = -0.0000000000002 * X^0"), (0, -0.)),
    (("2*X^0 = 2X^0"), (0, 0.)),
    (("2*X^0 = 2X^0"), (0, 0.)),
    (("--2*X^0 = 2X^0"), (0, 0.)),
    (("--+--2*X^0 = 2X^0"), (0, 0.)),
    (("--+--2 = --2X^0"), (0, 0.)),
];

#[test]
fn poly_parse_zero_degree_correct() -> Result<(), String> {
    let mut ret = Ok(());
    for case in POLY_PARSE_ZERO_DEGREE_CORRECT_TESTS {
        let poly = Polynomial {
            degrees: HashMap::<_, _>::from_iter(IntoIter::new([case.1]))
        };
        let poly_test = super::parse_polynomial(&case.0.to_owned())?;
        if poly == poly_test {
            ret = Ok(());
        }
        else {
            ret = Err(format!("Case: '{:?}', PT: {:?}, P: {:?}", case, poly_test.degrees, poly.degrees));
            break;
        }
    }
    ret
}

static POLY_PARSE_ZERO_DEGREE_FAIL_TESTS: &'static [&str] = &[
    "=2 = 2",
    "2 = 2=",
    " = 2",
    "0.123*$%^&* = 2",
    "*$%^&* = 2",
    // "2*= 2", // TODO: fix
    "*2= 2",
    "2..= 2",
    "2**X^0= 2",
    "2*X^^0= 2",
    "2*X^0.0= 2",
    "2*X^0.0= 2",
    // "2*X^0 --- = 2X^0", // TODO: fix
    "",
    "34567.678",
    "2234.779 * X^0",
    "2.2.22 *X^0 = X^0",
    // "2 = 2 * X^  000", // TODO: fix?
    // "2 = 2 * X^0001", // TODO: fix?
];

#[test]
fn poly_parse_zero_degree_fail() -> Result<(), String> {
    let mut ret = Ok(());
    for case in POLY_PARSE_ZERO_DEGREE_FAIL_TESTS {
        match super::parse_polynomial(&(*case).to_owned()) {
            Ok(poly) => {
                ret = Err(format!("Err expected, get polynomial. Case: {}, P: {:?}", case, poly));
                break;
            }
            Err(_err) => ret = Ok(()),
        }
    }
    ret
}
