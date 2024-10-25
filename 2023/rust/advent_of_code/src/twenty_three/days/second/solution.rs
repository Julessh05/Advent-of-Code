use std::collections::HashMap;
use std::hash::Hash;
use std::string::ToString;

use crate::twenty_three::days::shared::get_content;

/* FIRST PROBLEM */
/// Solves the first problem of the second december
pub fn first() {
    let content = strip_content(get_content("second"));
    let mut total: u64 = 0;
    for (index, game) in content.iter().enumerate() {
        if is_game_possible(game.as_str()) {
            total += index as u64 + 1;
        } else {
            continue;
        }
    }
    println!("Solution to the first problem of the second day: {}", total);
}

fn strip_content(unstripped: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for (index, line) in unstripped.iter().enumerate() {
        let i: String = (index + 1).to_string();
        let new_line = line.strip_prefix(format!("Game {i}: ").as_str()).unwrap();
        result.push(new_line.to_string());
    }
    return result;
}

fn is_game_possible(game: &str) -> bool {
    let draws = game.split(";");
    for draw in draws {
        let single_parts_of_draw = draw.split(",");
        for part in single_parts_of_draw {
            let mut stripped = part;
            if part.starts_with(" ") {
                stripped = part.strip_prefix(" ").unwrap();
            }
            let parts_of_single_draw: Vec<&str> = stripped.split(" ").collect();
            let count = parts_of_single_draw[0].parse::<u64>().unwrap();
            let color = parts_of_single_draw[1];
            for cc in COLOR_COMBINATIONS {
                if color == cc.color && count > cc.maximum_count {
                    return false;
                } else {
                    continue;
                }
            }
        }
    }
    return true;
}

#[derive(Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

struct ColorCombination<'a> {
    color: &'a str,
    maximum_count: u64,
}

const COLOR_COMBINATIONS: [ColorCombination; 3] = [
    ColorCombination {
        color: "red",
        maximum_count: 12,
    },
    ColorCombination {
        color: "green",
        maximum_count: 13,
    },
    ColorCombination {
        color: "blue",
        maximum_count: 14,
    },
];

/* SECOND PROBLEM */

/// Solves the second problem of the second day
pub fn second() {
    let content = strip_content(get_content("second"));
    let mapped: Vec<HashMap<Color, u64>> = split_into_map(content);
    let mut total: u64 = 0;
    for map in mapped {
        let power = map.get(&Color::Red).unwrap()
            * map.get(&Color::Green).unwrap()
            * map.get(&Color::Blue).unwrap();
        total += power;
    }
    println!(
        "Solution to the second problem of the second day: {}",
        total
    );
}

fn split_into_map(content: Vec<String>) -> Vec<HashMap<Color, u64>> {
    let mut result: Vec<HashMap<Color, u64>> = vec![];
    for game in content {
        let draws = game.split(";");
        let mut red_count: u64 = 0;
        let mut green_count: u64 = 0;
        let mut blue_count: u64 = 0;
        for draw in draws {
            let single_parts_of_draw = draw.split(",");
            for part in single_parts_of_draw {
                let mut stripped = part;
                if part.starts_with(" ") {
                    stripped = part.strip_prefix(" ").unwrap();
                }
                let parts_of_single_draw: Vec<&str> = stripped.split(" ").collect();
                let count = parts_of_single_draw[0].parse::<u64>().unwrap();
                let color = parts_of_single_draw[1];
                match color {
                    "red" => {
                        if count > red_count {
                            red_count = count
                        }
                    }
                    "green" => {
                        if count > green_count {
                            green_count = count
                        }
                    }
                    "blue" => {
                        if count > blue_count {
                            blue_count = count
                        }
                    }
                    _ => panic!("No color found"),
                }
            }
        }
        println!(
            "Red Count: {}, Green count: {}, blue Count: {}",
            red_count, green_count, blue_count
        );
        let mut map: HashMap<Color, u64> = HashMap::new();
        map.insert(Color::Red, red_count);
        map.insert(Color::Green, green_count);
        map.insert(Color::Blue, blue_count);
        result.push(map);
    }
    return result;
}
