use crate::first_december::second;

fn main() {
    second();
}

mod first_december {
    use std::fs;

    fn get_content() -> Vec<String> {
        let file_content = fs::read_to_string("src/01/content.txt").expect("Error");
        let vec: Vec<String> = file_content.split("\n").map(str::to_string).collect();
        return vec;
    }

    pub fn first() {
        let content = get_content();
        let mut total: u64 = 0;
        for line in content {
            total += get_total_numeric(line.to_string());
        }
        println!("{}", total);
    }

    fn get_first_and_last_numeric(input: String) -> Vec<char> {
        let mut one: char = 'A';
        let mut two: char = 'B';
        let mut is_first: bool = true;
        for char in input.chars() {
            if char.is_numeric() {
                if is_first {
                    is_first = false;
                    one = char;
                }
                two = char;
            }
        }
        return Vec::from([one, two]);
    }

    fn get_total_numeric(input: String) -> u64 {
        let vec: Vec<char> = get_first_and_last_numeric(input);
        let mut string = vec[0].to_string();
        string.push(vec[1]);
        return string.parse::<u64>().unwrap();
    }

    pub fn second() {
        let content = get_content();
        let mut total: u64 = 0;
        for line in content {
            if contains_alphabetic_number(line.to_string()) {
                let first_and_last_alphabetic: Vec<&str> = get_first_and_last_alphabetic_number(line);
            } else {
                total += get_total_numeric(line);
            }
        }
    }

    #[derive(Clone, Copy)]
    struct AlphabeticNumber<'a> {
        alphabetic: &'a str,
        numeric: &'a str,
    }


    static ALPHABETIC_NUMBERS: [AlphabeticNumber; 9] = [
        AlphabeticNumber {
            alphabetic: "one",
            numeric: "1",
        },
        AlphabeticNumber {
            alphabetic: "two",
            numeric: "2",
        },
        AlphabeticNumber {
            alphabetic: "three",
            numeric: "3",
        },
        AlphabeticNumber {
            alphabetic: "four",
            numeric: "4",
        },
        AlphabeticNumber {
            alphabetic: "five",
            numeric: "5",
        },
        AlphabeticNumber {
            alphabetic: "six",
            numeric: "6",
        },
        AlphabeticNumber {
            alphabetic: "seven",
            numeric: "7",
        },
        AlphabeticNumber {
            alphabetic: "eight",
            numeric: "8",
        },
        AlphabeticNumber {
            alphabetic: "nine",
            numeric: "9",
        },
    ];

    fn contains_alphabetic_number(input: String) -> bool {
        for an in ALPHABETIC_NUMBERS {
            if input.contains(an.alphabetic) { return true; } else { continue; }
        }
        return false;
    }

    fn get_first_and_last_alphabetic_number<'a>(input: String) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        for an in ALPHABETIC_NUMBERS {
            if input.contains(an.alphabetic) {
                if result.len() < 2 {
                    result.push(an.numeric)
                } else {
                    result[1] = an.numeric;
                }
            } else {
                continue;
            }
        }
        return result;
    }

    enum NumberType {
        Alphabetic,
        Numeric,
    }

    fn get_first_and_last_type(input: String) -> Vec<NumberType> {
        let result: Vec<NumberType> = Vec::new();
        let first_index_numeric: u64;
        let last_index_numeric: u64;
        let mut is_first_numeric: bool = true;
        let first_index_alphabetic: u64;
        let last_index_alphabetic: u64;
        let mut is_first_alphabetic: bool = true;
        for (index, char) in input.chars().enumerate() {
            if char.is_numeric() {
                if is_first_numeric {
                    first_index_numeric = index as u64;
                }
                last_index_numeric = index as u64;
            }
        }
        return result;
    }
}