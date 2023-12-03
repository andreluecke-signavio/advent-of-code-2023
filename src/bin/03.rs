use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(3);

type SpecialChars = HashMap<Coord, char>;
type Numbers = Vec<Vec<(Coord, u32)>>;
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (special_chars, numbers) = analyze_grid(input);

    let parts = numbers
        .iter()
        .filter(|digits| {
            digits
                .iter()
                .any(|(coord, _)| check_digit(coord, &special_chars))
        })
        .map(|digits| convert_to_part(digits))
        .collect::<Vec<u32>>();
    Some(parts.iter().sum())
}

fn convert_to_part(digits: &[(Coord, u32)]) -> u32 {
    let digits: Vec<u32> = digits.iter().map(|(_, val)| *val).collect();
    digits
        .into_iter()
        .map(|digit| digit.to_string())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn analyze_grid(input: &str) -> (SpecialChars, Numbers) {
    let mut special_chars: HashMap<Coord, char> = HashMap::new();
    let mut numbers: Vec<Vec<(Coord, u32)>> = Vec::new();

    let mut digits = vec![];
    input.lines().enumerate().for_each(|(row, line)| {
        let mut is_number = false;
        digits.clear();

        line.trim().chars().enumerate().for_each(|(col, char)| {
            let coord = Coord {
                x: col as i32,
                y: row as i32,
            };
            if char.is_ascii_digit() {
                is_number = true;
                digits.push((coord, char.to_digit(10).unwrap()));
            } else if !char.is_ascii_digit() {
                if is_number {
                    // end of number reached
                    is_number = false;
                    numbers.push(digits.clone());
                    digits.clear();
                }
                if !char.is_ascii_digit() && !char.is_ascii_whitespace() && char != '.' {
                    special_chars.insert(coord, char);
                }
            }
        });
        if !digits.is_empty() {
            numbers.push(digits.clone());
        }
    });
    (special_chars, numbers)
}

fn check_digit(coord: &Coord, special_chars: &HashMap<Coord, char>) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if special_chars.contains_key(&Coord {
                x: coord.x + i,
                y: coord.y + j,
            }) {
                return true;
            }
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let (special_chars, numbers) = analyze_grid(input);

    let mut gear_adjacencies = HashMap::new();

    special_chars
        .iter()
        .filter(|(_, char)| **char == '*')
        .for_each(|(coord, _)| {
            for i in -1..=1 {
                for j in -1..=1 {
                    gear_adjacencies.insert(
                        Coord {
                            x: coord.x + i,
                            y: coord.y + j,
                        },
                        *coord,
                    );
                }
            }
        });

    let mut valid_gears: HashMap<Coord, HashSet<Vec<(Coord, u32)>>> = HashMap::new();

    numbers.iter().for_each(|digits| {
        digits.iter().for_each(|(digit_coord, _)| {
            // if a digit is next to a gear ('*') collect the whole digits vector and associate it with the gear
            if let Some(gear_coord) = gear_adjacencies.get(digit_coord) {
                if let Some(adjacent_numbers) = valid_gears.get_mut(gear_coord) {
                    adjacent_numbers.insert(digits.clone());
                } else {
                    let mut new_adjacent_numbers = HashSet::new();
                    new_adjacent_numbers.insert(digits.clone());
                    valid_gears.insert(*gear_coord, new_adjacent_numbers);
                }
            }
        });
    });

    // since we have now all the gears with all their adjacent numbers, we can compute the gear ratios and sum them up
    let sum = valid_gears
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| {
            let numbers = numbers.iter().collect::<Vec<&Vec<(Coord, u32)>>>();
            let first = convert_to_part(numbers[0].as_slice());
            let second = convert_to_part(numbers[1].as_slice());
            first * second
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
