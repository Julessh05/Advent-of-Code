use std::iter::Map;
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

/// Strip unnecessary content from the input
fn strip_content_with_pointer(unstripped: &Vec<String>) {
    for (index, line) in unstripped.iter().enumerate() {
        let i: String = (index + 1).to_string();
        line.strip_prefix(format!("Game {i}: ").as_str()).unwrap();
    }
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
pub fn second() {
    let content = strip_content(get_content("second"));
}

fn split_into_map(content: Vec<String>) -> Vec<Map<Color, u64>> {
    let mut result: Vec<Map<Color, u64>> = vec![];
    for game in content {
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
            }
        }
    }
    return result;
}
