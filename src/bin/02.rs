advent_of_code::solution!(2);

#[derive(PartialEq, Debug)]
struct Game {
    game: u32,
    sets: Vec<CubeSet>,
}

type CubeSet = Vec<(u32, Color)>;

#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(PartialEq, Debug)]
struct MinimumSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl MinimumSet {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn part_one(input: &str) -> Option<u32> {
    let sum_of_valid_game_numbers = input
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .filter(evaluate_game)
        .map(|game| game.game)
        .sum();

    Some(sum_of_valid_game_numbers)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|g| calculate_minimum_set(&g))
        .map(|m| m.power())
        .sum();

    Some(sum)
}

fn parse_line(line: &str) -> Result<Game, String> {
    let parts: Vec<&str> = line.split(':').collect();

    if parts.len() != 2 {
        return Err("Invalid line format".to_string());
    }

    let game_number = parts[0]
        .split_whitespace()
        .last()
        .ok_or("Invalid game number")?
        .parse::<u32>()
        .map_err(|_| "Invalid game number".to_string())?;

    let game_colors: Result<Vec<Vec<(u32, Color)>>, String> = parts[1]
        .trim()
        .split(';')
        .map(|group| {
            group
                .trim()
                .split(',')
                .map(|item| {
                    let item_parts: Vec<&str> = item.trim().split(' ').collect();
                    if item_parts.len() != 2 {
                        return Err("Invalid color format".to_string());
                    }
                    let number = item_parts[0]
                        .parse::<u32>()
                        .map_err(|_| "Invalid number".to_string())?;
                    let color = match item_parts[1] {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => return Err("Invalid color".to_string()),
                    };
                    Ok((number, color))
                })
                .collect::<Result<Vec<(u32, Color)>, String>>()
        })
        .collect();

    let game_colors = game_colors?;

    Ok(Game {
        game: game_number,
        sets: game_colors,
    })
}

fn evaluate_game(game: &Game) -> bool {
    for cube_set in &game.sets {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for (number, color) in cube_set {
            match color {
                Color::Red => red_count += number,
                Color::Green => green_count += number,
                Color::Blue => blue_count += number,
            }
        }

        if red_count > 12 || green_count > 13 || blue_count > 14 {
            return false;
        }
    }

    true
}

fn calculate_minimum_set(game: &Game) -> MinimumSet {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for cube_set in &game.sets {
        for (number, color) in cube_set {
            match color {
                Color::Red => max_red = max_red.max(*number),
                Color::Green => max_green = max_green.max(*number),
                Color::Blue => max_blue = max_blue.max(*number),
            }
        }
    }

    MinimumSet {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_line() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let result = parse_line(line).unwrap();
        assert_eq!(
            result,
            Game {
                game: 3,
                sets: vec![
                    vec![(8, Color::Green), (6, Color::Blue), (20, Color::Red)],
                    vec![(5, Color::Blue), (4, Color::Red), (13, Color::Green)],
                    vec![(5, Color::Green), (1, Color::Red)],
                ],
            }
        );
    }

    #[test]
    fn evaluate_single_game() {
        let game = Game {
            game: 3,
            sets: vec![
                vec![(8, Color::Green), (6, Color::Blue), (20, Color::Red)],
                vec![(5, Color::Blue), (4, Color::Red), (13, Color::Green)],
                vec![(5, Color::Green), (1, Color::Red)],
            ],
        };

        let result = evaluate_game(&game);
        assert_eq!(result, false);
    }

    #[test]
    fn compute_minimum_set_for_single_game() {
        let game =
            parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
                .unwrap();
        let result = calculate_minimum_set(&game);
        assert_eq!(
            result,
            MinimumSet {
                red: 20,
                blue: 6,
                green: 13,
            }
        );
    }

    #[test]
    fn power_of_examples() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let games = input
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<Game>, String>>()
            .unwrap();

        let result: u32 = games
            .iter()
            .map(|g| calculate_minimum_set(g))
            .map(|m| m.power())
            .sum();

        assert_eq!(result, 2286)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
