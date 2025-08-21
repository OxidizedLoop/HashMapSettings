use crate::account::Incrementable;

impl Incrementable for i8 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for i16 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for i32 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for i64 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for i128 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for isize {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for u8 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for u16 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for u32 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for u64 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for u128 {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for usize {
    fn increment_mut(&mut self) {
        *self = self.increment();
    }
    fn increment(&self) -> Self {
        self.overflowing_add(1).0
    }
}
impl Incrementable for String {
    fn increment_mut(&mut self) {
        let mut chars = self.chars().rev();
        let mut position: u32 = 2;
        let mut r_value = Self::new();
        if chars.nth(0) == Some(')')
            && let Some(char) = chars.nth(0)
            && char.is_ascii_digit()
        {
            loop {
                match chars.nth(0) {
                    Some(x) if x.is_ascii_digit() => {
                        position.increment_mut();
                        continue;
                    }
                    Some('(') => (),
                    Some(_) | None => break,
                }
                //add number
                let (name, number) = self.rsplit_once('(').unwrap();
                let number: i32 = number
                    .trim_end_matches(')')
                    .parse::<i32>()
                    .unwrap() //safe unwrap, or we wouldn't reach this point
                    .increment();
                r_value = name.to_string() + "(" + &number.to_string() + ")";
                break;
            }
            if !r_value.is_empty() {
                *self = r_value;
                return;
            }
        }
        self.push_str("(1)");
    }
    fn increment(&self) -> Self {
        let mut chars = self.chars().rev();
        let mut position: u32 = 2;
        if chars.nth(0) == Some(')')
            && let Some(char) = chars.nth(0)
            && char.is_ascii_digit()
        {
            loop {
                match chars.nth(0) {
                    Some(x) if x.is_ascii_digit() => {
                        position.increment_mut();
                        continue;
                    }
                    Some('(') => (),
                    Some(_) | None => break,
                }
                //add number
                let (name, number) = self.rsplit_once('(').unwrap();
                let number: i32 = number
                    .trim_end_matches(')')
                    .parse::<i32>()
                    .unwrap() //safe unwrap, or we wouldn't reach this point
                    .increment();
                return name.to_string() + "(" + &number.to_string() + ")";
            }
        }
        let mut r_value = self.clone();
        r_value.push_str("(1)");
        r_value
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Incrementable;

    #[test]
    fn i32_overflow() {
        let mut max = i32::MAX;
        max.increment_mut();
        assert_eq!(max, i32::MIN);
    }
    #[test]
    fn u32_overflow() {
        let mut max = i32::MAX;
        max.increment_mut();
        assert_eq!(max, i32::MIN);
    }
    #[test]
    fn empty_string() {
        let mut string: String = String::new();
        string.increment_mut();
        assert_eq!(string, "(1)");
    }
    #[test]
    fn normal_string() {
        let mut string: String = "normal".to_string();
        string.increment_mut();
        assert_eq!(string, "normal(1)");
    }
    #[test]
    fn incremented_string() {
        let mut string: String = "incremented(1)".to_string();
        string.increment_mut();
        assert_eq!(string, "incremented(2)");
    }
    #[test]
    fn parenthesis_string() {
        let mut string: String = "(parenthesis(1)".to_string();
        string.increment_mut();
        assert_eq!(string, "(parenthesis(2)");
    }
    #[test]
    fn nine_string() {
        let mut string: String = "nine(9)".to_string();
        string.increment_mut();
        assert_eq!(string, "nine(10)");
    }
}
