use std::string::ToString;

use crate::twenty_three::days::shared;

/* FIRST PROBLEM */
/// Solves the first problem of the second december
pub fn first() {
    let content = strip_content(shared::get_content("second"));
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
    println!("Draws: {:?}", draws);
    for draw in draws {
        println!("One draw: {}", draw);
        let single_parts_of_draw = draw.split(",");
        println!("All single parts of this draw: {:?}", single_parts_of_draw);
        for part in single_parts_of_draw {
            println!("A single part: {}", part);
            let mut stripped = part;
            if part.starts_with(" ") {
                stripped = part.strip_prefix(" ").unwrap();
            }
            let parts_of_single_draw: Vec<&str> = stripped.split(" ").collect();
            let count = parts_of_single_draw[0].parse::<u64>().unwrap();
            let color = parts_of_single_draw[1];
            println!("Color: {}", color);
            println!("Count: {}", count);
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
pub fn second() {}
