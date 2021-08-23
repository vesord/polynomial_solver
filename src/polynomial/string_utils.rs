pub trait StringUtils {
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

