advent_of_code::solution!(1);

trait ReverseDigits {
    fn reverse_words(&self) -> Vec<(String, u32)>;
}

impl ReverseDigits for Vec<(String, u32)> {
    fn reverse_words(&self) -> Vec<(String, u32)> {
        self.iter()
            .map(|(s, v)| (s.chars().rev().collect::<String>(), *v))
            .collect()
    }
}

struct DigitMappings {
    mappings: Vec<(String, u32)>,
    reverse_mappings: Vec<(String, u32)>,
}

impl DigitMappings {
    fn new(mappings: Vec<(String, u32)>) -> Self {
        Self {
            reverse_mappings: mappings.reverse_words(),
            mappings,
        }
    }

    fn extend(&mut self, mappings: Vec<(String, u32)>) {
        self.mappings.extend(mappings);
        self.reverse_mappings = self.mappings.reverse_words();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    sum_digits(input, part_one_digits())
}

pub fn part_two(input: &str) -> Option<u32> {
    sum_digits(input, part_two_digits())
}

fn sum_digits(input: &str, digit_mappings: DigitMappings) -> Option<u32> {
    let sum = input
        .lines()
        .filter_map(|line| extract_digits_from_line(line, &digit_mappings))
        .sum::<u32>();

    if sum > 0 {
        Some(sum)
    } else {
        None
    }
}

fn extract_digits_from_line(line: &str, digit_mappings: &DigitMappings) -> Option<u32> {
    let first_digit = find_first_digit(line, &digit_mappings.mappings)?;

    let line_reversed = line.chars().rev().collect::<String>();
    let last_digit = find_first_digit(&line_reversed, &digit_mappings.reverse_mappings)?;

    Some(first_digit * 10 + last_digit)
}

fn find_first_digit(line: &str, digits: &Vec<(String, u32)>) -> Option<u32> {
    let mut lowest_index = usize::MAX;
    let mut first_digit = 0;

    for (digit_str, digit_val) in digits {
        if let Some(index) = line.find(digit_str) {
            if index < lowest_index {
                lowest_index = index;
                first_digit = *digit_val;
            }
        }
    }

    if lowest_index == usize::MAX {
        None
    } else {
        Some(first_digit)
    }
}

fn part_one_digits() -> DigitMappings {
    DigitMappings::new(vec![
        ("0".to_string(), 0),
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
    ])
}

fn part_two_digits() -> DigitMappings {
    let mut digits = part_one_digits();
    digits.extend(vec![
        ("zero".to_string(), 0),
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_single_line() {
        let result = extract_digits_from_line("1abc2", &part_one_digits());
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_one_no_digits() {
        let result = extract_digits_from_line("abcde", &part_one_digits());
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_digits_anywhere() {
        let result = extract_digits_from_line("abc1def2", &part_one_digits());
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_one_single_digit() {
        let result = extract_digits_from_line("treb7uchet", &part_one_digits());
        assert_eq!(result, Some(77));
    }

    #[test]
    fn test_part_one_empty_string() {
        let result = extract_digits_from_line("", &part_one_digits());
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_all_digits() {
        let result = extract_digits_from_line("12345", &part_one_digits());
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_one_special_characters() {
        let result = extract_digits_from_line("\t^3!@#$4´", &part_one_digits());
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_one_zeros() {
        let result = extract_digits_from_line("0abc0", &part_one_digits());
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_extract_digits_spelled_out_digits() {
        let result = extract_digits_from_line("two1nine", &part_two_digits());
        assert_eq!(result, Some(29));
    }

    #[test]
    fn test_extract_digits_spelled_out_digit_at_beginning_number_at_end() {
        let result = extract_digits_from_line("four1", &part_two_digits());
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_extract_digits_digit_at_beginning_spelled_out_digit_at_end() {
        let result = extract_digits_from_line("7eight", &part_two_digits());
        assert_eq!(result, Some(78));
    }

    #[test]
    fn test_extract_digits_both_digits_spelled_out() {
        let result = extract_digits_from_line("sixseven", &part_two_digits());
        assert_eq!(result, Some(67));
    }

    #[test]
    fn test_extract_digits_mixed_with_spelled_out() {
        let result = extract_digits_from_line("a3bctwodfive", &part_two_digits());
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_extract_digits_spelled_out_with_words_in_between() {
        let result = extract_digits_from_line("oneabc3defgfour", &part_two_digits());
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_extract_digits_overlapping_spelled_out_digits() {
        let result = extract_digits_from_line("eightwothree", &part_two_digits());
        assert_eq!(result, Some(83));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#,
        );
        assert_eq!(result, Some(281));
    }
}
