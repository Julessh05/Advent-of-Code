use crate::twenty_three::days::shared::get_content;

pub fn first() {
    let content: Vec<String> = get_content("third");
    let content_as_chars: Vec<Vec<char>> = content_to_chars(content);
    let mut total: u64 = 0;
    for line_index in 0..content_as_chars.len() {
        let mut start_index: i64 = -1;
        let mut end_index: i64 = -1;
        let mut is_connected: bool = false;
        for char_index in 0..content_as_chars[line_index].len() {
            if content_as_chars[line_index][char_index].is_numeric() {
                if start_index == -1 {
                    start_index = char_index as i64;
                }
                for y_loop in line_index..line_index + 2 {
                    let y: i64 = y_loop as i64 - 1;
                    if y < 0 || y >= content_as_chars.len() as i64 {
                        continue;
                    }
                    for x_loop in char_index..=char_index + 2 {
                        let x: i64 = x_loop as i64 - 1;
                        if x < 0 || x >= content_as_chars[y as usize].len() as i64 {
                            continue;
                        }
                        if !content_as_chars[y as usize][x as usize].is_numeric()
                            && content_as_chars[y as usize][x as usize] != '.'
                        {
                            is_connected = true;
                        } else {
                            continue;
                        }
                    }
                }
            } else {
                if start_index != -1 {
                    end_index = char_index as i64 - 1;
                    if is_connected {
                        let mut number_as_string: String = String::new();
                        for i in start_index..end_index {
                            number_as_string.push(content_as_chars[line_index][i as usize])
                        }
                        println!("Start index: {}, end index: {}", start_index, end_index);
                        println!("Line Index: {}", line_index);
                        println!("{}", number_as_string);
                        let number = number_as_string.parse::<u64>().unwrap();
                        is_connected = false;
                        total += number;
                    }
                    start_index = -1;
                    end_index = -1;
                } else {
                    continue;
                }
            }
        }
    }
    println!("Solution to the first problem of the third day: {}", total);
}

fn content_to_chars(content: Vec<String>) -> Vec<Vec<char>> {
    let mut chars: Vec<Vec<char>> = vec![];
    for line in content {
        let mut char_line: Vec<char> = vec![];
        for char in line.chars() {
            char_line.push(char);
        }
        chars.push(char_line);
    }
    return chars;
}
