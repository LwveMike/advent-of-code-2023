use std::fs;

pub fn main() {
    let output = process();

    println!("{}", output);
}


fn process() -> usize {
   let content = fs::read_to_string("d1i1").expect("Should have string content");

   let input = content.split_whitespace();

    let mut char_digits = vec!();
    
    for line in input {
        let mut first_digit: char = '0';
        let mut last_digit: char = '0';

        for ch in line.chars() {
            if ch.is_digit(10) {
               first_digit = ch;
               break
            }
        };

        for ch in line.chars().rev() {
            if ch.is_digit(10) {
               last_digit = ch;
               break
            }
        };

        char_digits.push(format!("{}{}", first_digit, last_digit));
    }

    let mut result = 0_usize;

    for string_number in &char_digits {
        let number: usize = string_number.parse().expect("idk");

        result += number;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::part01::process;

    #[test]
    fn output () {
        let output = process();

        assert_eq!(output, 142);
    }
}










