pub trait F64Utils {
    fn ft_sqrt(self) -> f64;
    fn positivize_zero(self) -> f64;
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

    fn positivize_zero(self) -> f64 {
        if self.abs() > 0. { return self }
        if self == -0. { return 0. }
        self
    }
}