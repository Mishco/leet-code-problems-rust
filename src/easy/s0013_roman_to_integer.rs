#![allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut stream = s.chars().peekable();

    while let Some(current) = stream.next() {
        let next = stream.peek();
        result += match (current, next) {
            ('I', Some('V')) => -1,
            ('I', Some('X')) => -1,
            ('X', Some('L')) => -10,
            ('X', Some('C')) => -10,
            ('C', Some('D')) => -100,
            ('C', Some('M')) => -100,
            ('I', _) => 1,
            ('V', _) => 5,
            ('X', _) => 10,
            ('L', _) => 50,
            ('C', _) => 100,
            ('D', _) => 500,
            ('M', _) => 1000,
            _ => panic!("Bad symbol")
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("IX")), 9);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}