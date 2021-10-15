#[cfg(test)]

mod tests {
    use std::iter::FromIterator;
    use std::array::IntoIter;
    use std::collections::HashMap;
    use float_eq;
    use float_eq::FloatEqAll;
    use crate::polynomial::Polynomial;
    use crate::parse_polynomial;

    impl std::cmp::PartialEq for Polynomial {
        fn eq(&self, other: &Self) -> bool {
            self.degrees.eq_rmax_all(&other.degrees, &(f64::EPSILON * 10.))
        }
    }

    static POLY_PARSE_ZERO_DEGREE_CORRECT_TESTS: &'static [(&str, (i32, f64))] = &[
        (("7 = 7"), (0, 0.)),
        (("0 = 0"), (0, 0.)),
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
            let poly_test = parse_polynomial(&case.0.to_owned())?;
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

    static POLY_PARSE_FIRST_DEGREE_CORRECT_TESTS: &'static [(&str, (i32, f64), (i32, f64))] = &[
        (("7 * X ^ 1 + 0 = 7 * X ^ 1"), (0, 0.), (1, 0.)),
        (("0 * X ^ 1 + 0 = 7 * X ^ 1"), (0, 0.), (1, -7.)),
        (("0 * X ^ 1 + 0 = 0 * X ^ 1"), (0, 0.), (1, 0.)),
        (("5 * X + 0 = 0 * X ^ 1"), (0, 0.), (1, 5.)),
        (("5X + 0 = 0X ^ 1"), (0, 0.), (1, 5.)),
        (("5X + 3 = 0X ^ 1"), (0, 3.), (1, 5.)),
        (("5X + 3 = 0X ^ 1 - 3"), (0, 6.), (1, 5.)),
        (("5X + 3 = 0X ^ 1 + 3"), (0, 0.), (1, 5.)),
        (("5X + 3 = 7 * X + 3"), (0, 0.), (1, -2.)),
        (("5X = 7 * X + 3"), (0, -3.), (1, -2.)),
        (("0X = 7 * X + 3"), (0, -3.), (1, -7.)),
        (("0 = 7 * X + 3"), (0, -3.), (1, -7.)),
        (("7 * X + 3 = 0"), (0, 3.), (1, 7.)),
        (("X = 0"), (0, 0.), (1, 1.)),
        (("X = X + 0"), (0, 0.), (1, 0.)),
        (("0.23X = 0.11X + 0"), (0, 0.), (1, 0.12)),
        (("X = 78"), (0, -78.), (1, 1.)),
    ];

    #[test]
    fn poly_parse_first_degree_correct() -> Result<(), String> {
        let mut ret = Ok(());
        for case in POLY_PARSE_FIRST_DEGREE_CORRECT_TESTS {
            let poly = Polynomial {
                degrees: HashMap::<_, _>::from_iter(IntoIter::new([case.1, case.2]))
            };
            let poly_test = parse_polynomial(&case.0.to_owned())?;
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

    static POLY_PARSE_SECOND_DEGREE_CORRECT_TESTS: &'static [(&str, (i32, f64), (i32, f64), (i32, f64))] = &[
        (("3 * X ^ 2 + 7 * X ^ 1 + 0 = 7 * X ^ 1"), (0, 0.), (1, 0.), (2, 3.)),
        (("0 * X ^ 2 + 0 * X ^ 1 + 0 = 7 * X ^ 1"), (0, 0.), (1, -7.), (2, 0.)),
        (("0 * X ^ 2 + 0 * X ^ 1 + 0 = 0 * X ^ 1"), (0, 0.), (1, 0.), (2, 0.)),
        (("0.05 * X ^ 2 + 5 * X + 0 = 0 * X ^ 1"), (0, 0.), (1, 5.), (2, 0.05)),
        (("12.22X^2 + 5X + 0 = 0X ^ 1"), (0, 0.), (1, 5.), (2, 12.22)),
        (("-12.22X^2 + 5X + 0 = 0X ^ 1"), (0, 0.), (1, 5.), (2, -12.22)),
        (("5X + 0 = 0X ^ 1 -12.22X^2"), (0, 0.), (1, 5.), (2, 12.22)),
        (("-12.22X^2 + 5X + 0 = 0X ^ 1 -12.22X^2"), (0, 0.), (1, 5.), (2, 0.)),
        (("0 = 0X ^ 1 -12.22X^2"), (0, 0.), (1, 0.), (2, 12.22)),
        (("0 = 0X ^ 1 -0.X^2"), (0, 0.), (1, 0.), (2, 0.0)),
        (("0 = 0X ^ 1 -0.0X^2"), (0, 0.), (1, 0.), (2, 0.)),
    ];

    #[test]
    fn poly_parse_second_degree_correct() -> Result<(), String> {
        let mut ret = Ok(());
        for case in POLY_PARSE_SECOND_DEGREE_CORRECT_TESTS {
            let poly = Polynomial {
                degrees: HashMap::<_, _>::from_iter(IntoIter::new([case.1, case.2, case.3]))
            };
            let poly_test = parse_polynomial(&case.0.to_owned())?;
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

    static POLY_PARSE_ANY_DEGREE_CORRECT_TESTS: &'static [(&str, (i32, f64), (i32, f64), (i32, f64))] = &[
        (("3 * X ^ 22 + 7 * X ^ 1 + 0 = 7 * X ^ 1"), (0, 0.), (1, 0.), (22, 3.)),
        (("0 * X ^ 82 + 0 * X ^ 1 + 0 = 7 * X ^ 1"), (0, 0.), (1, -7.), (82, 0.)),
        (("1 * X ^ 82 + 0 * X ^ 1 + 0 = 7 * X ^ 1 - 11 * X ^ 82 "), (0, 0.), (1, -7.), (82, 12.)),
        (("-X^12 + X^89 + 12.42X^12 - X^99 + 4X^99  = -5X^99 + 4X^89 -6X^12 -0.5X^89 "), (12, 17.42), (89, -2.5), (99, 8.)),
    ];


    #[test]
    fn poly_parse_any_degree_correct() -> Result<(), String> {
        let mut ret = Ok(());
        for case in POLY_PARSE_ANY_DEGREE_CORRECT_TESTS {
            let poly = Polynomial {
                degrees: HashMap::<_, _>::from_iter(IntoIter::new([case.1, case.2, case.3]))
            };
            let poly_test = parse_polynomial(&case.0.to_owned())?;
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

    static POLY_PARSE_FAIL_TESTS: &'static [&str] = &[
        "=2 = 2",
        "2 = 2=",
        " = 2",
        "0.123*$%^&* = 2",
        "*$%^&* = 2",
        "2*= 2",
        "2XX = 2",
        "X2X = 2",
        "*2= 2",
        "2..= 2",
        "2**X^0= 2",
        "2*X^^0= 2",
        "2*X^0.0= 2",
        "2*X^0.0= 2",
        "",
        "34567.678",
        "2234.779 * X^0",
        "2.2.22 *X^0 = X^0",
        "XX = XX",
        "=",
        "*",
        "-",
        "+",
        "-*=+*",
        "X^0.2 = X",
        "X^2X = X ^ 2",
        "0X = 0.0+.0.X",
    ];

    #[test]
    fn poly_parse_fail() -> Result<(), String> {
        let mut ret = Ok(());
        for case in POLY_PARSE_FAIL_TESTS {
            match parse_polynomial(&(*case).to_owned()) {
                Ok(poly) => {
                    ret = Err(format!("Err expected, get polynomial. Case: {}, P: {:?}", case, poly));
                    break;
                }
                Err(_err) => ret = Ok(()),
            }
        }
        ret
    }

    static POLY_SOLVE_VALID_TESTS: &'static [(&str, &str)] = &[
        ("0X^2 + 0X + 0 = 0", "No solution, polynomial is correct"),
        ("0X^2 + 0X + 0.000 = 0", "No solution, polynomial is correct"),
        ("0X^2 + 0X + 1.01 = 0", "No solution, polynomial is incorrect"),
        ("0X^2 + 0X - 1 = 0", "No solution, polynomial is incorrect"),
        ("0X^2 + 1X + 0 = 0", "0"),
        ("0X^2 + 1X + 1 = 0", "-1"),
        ("0X^2 + 1X - 1 = 0", "1"),
        ("0X^2 + 4X - 1 = 0", "0.25"),
        ("0X^2 + 1958754X + 85489513218 = 0", "-43644.84423158804"),
        ("0X^2 + 1958754X - 85489513218 = 0", "43644.84423158804"),
        ("1X^2 + 0X - 16 = 0", "4,-4"),
        ("1X^2 + 0X - 0 = 0", "0,0"),
        ("1X^2 + 0X - 15241.5787502 = 0", "123.45678900003838,-123.45678900003838"),
        ("1X^2 - 1X + 0.25 = 0", "0.5,0.5"),
        ("1X^2 - 2X + 1 = 0", "1,1"),
        ("1X^2 - 3X + 2.25 = 0", "1.5,1.5"),
        ("1X^2 + 1X + 0.25 = 0", "-0.5,-0.5"),
        ("1X^2 + 2X + 1 = 0", "-1,-1"),
        ("1X^2 + 3X + 2.25 = 0", "-1.5,-1.5"),
        ("1X^2 + 1X - 0.25 = 0", "0.20710678118654746,-1.2071067811865475"),
        ("1X^2 + 3X - 2.25 = 0", "0.6213203435596428,-3.621320343559643"),
        ("1X^2 + 78X - 98 = 0", "1.2367990774614128,-79.23679907746141"),
        ("1X^2 + 7123456788X - 91234567898 = 0", "12.807625770568848,-7123456800.807626"),
        ("1X^2 + 2X + 42 = 0", "-1+6.4031242374328485i,-1-6.4031242374328485i"),
        ("1X^2 + 222X + 424242 = 0", "-111+641.8107197608965i,-111-641.8107197608965i"),
        ("1X^2 + 0X + 16 = 0", "0+4i,0-4i"),
        ("1X^2 + 0X + 15241.5787502 = 0", "0+123.45678900003838i,0-123.45678900003838i"),

        ("5 * X^0 = 5 * X^0", "No solution, polynomial is correct"),
        ("5 * X^0 = 4 * X^0 + 7 * X^1", "0.14285714285714285"),
        ("5 * X^0 + 13 * X^1 + X^2 = 1 * X^0 + 1 * X^1", "-0.3431457505076203,-11.65685424949238"),
        ("6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0 + 1 * X^1", "-1,-1"),
        ("5 * X^0 + 3 * X^1 + 3 * X^2 = 1 * X^0 + 0 * X^1", "-0.5+1.0408329997330663i,-0.5-1.0408329997330663i"),
    ];

    #[test]
    fn poly_solve() -> Result<(), String> {
        let mut ret = Ok(());
        for case in POLY_SOLVE_VALID_TESTS {
            let poly = parse_polynomial(&(*case).0.to_owned()).unwrap();
            match poly.solve() {
                Ok(res) => {
                    if res != case.1 { ret = Err(format!("Wrong solution. Expect: {}, Res: {}", case.1, res))}
                }
                Err(err) => {
                    ret = Err(err);
                    break;
                }
            }
        }
        ret
    }
}
