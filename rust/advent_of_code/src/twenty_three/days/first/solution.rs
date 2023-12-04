mod solution {
    use std::fs;
    use std::str;

    /// Get Content of File and returns lines
    fn get_content() -> Vec<String> {
        let file_content = fs::read_to_string("content.txt").expect("Error reading content");
        let mut vec: Vec<String> = file_content.split("\n").map(str::to_string).collect();
        vec.retain(|line| !line.is_empty());
        return vec;
    }

    /// The first problem of this day
    pub fn _first() {
        let content = get_content();
        let mut total: u64 = 0;
        for line in content {
            total += get_total_numeric(line.to_string());
        }
        println!("{}", total);
    }

    /// Returns the first and last numeric number
    fn get_first_and_last_numeric(input: &String) -> Vec<char> {
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

    /// Returns the total of the first and last number in
    /// a specific string
    fn get_total_numeric(input: String) -> u64 {
        let vec: Vec<char> = get_first_and_last_numeric(&input);
        let mut string = vec[0].to_string();
        string.push(vec[1]);
        return string.parse::<u64>().unwrap();
    }

    /// Method to run in order to solve the second problem
    pub fn second() {
        let content = get_content();
        let mut total: u64 = 0;
        for line in content {
            // Not needed, as empty lines are filtered out in get_content since 04.12.twenty_three
            //if line.is_empty() { continue; }
            if contains_alphabetic_number(&line.to_string()) {
                let first_and_last_alphabetic: Vec<&str> =
                    get_first_and_last_alphabetic_number(&line);
                let types = get_first_and_last_type(&line);
                let mut first_and_last_numeric: Vec<char> = vec![];
                println!("Line: {:?}", line);
                println!("First and last alpha: {:?}", first_and_last_alphabetic);
                println!("Types: {:?}", types);
                if types.contains(&NumberType::Numeric) {
                    first_and_last_numeric = get_first_and_last_numeric(&line);
                }
                println!("First and last numeric: {:?}", first_and_last_numeric);
                let mut first_and_last: Vec<u64> = vec![];
                if matches!(types[0], NumberType::Alphabetic) {
                    first_and_last.push(
                        first_and_last_alphabetic[0]
                            .to_string()
                            .parse::<u64>()
                            .unwrap(),
                    );
                } else {
                    first_and_last.push(
                        first_and_last_numeric[0]
                            .to_string()
                            .parse::<u64>()
                            .unwrap(),
                    );
                }
                if matches!(types[1], NumberType::Alphabetic) {
                    first_and_last.push(
                        first_and_last_alphabetic[1]
                            .to_string()
                            .parse::<u64>()
                            .unwrap(),
                    );
                } else {
                    first_and_last.push(
                        first_and_last_numeric[1]
                            .to_string()
                            .parse::<u64>()
                            .unwrap(),
                    );
                }
                let mut string: String = first_and_last[0].to_string();
                string.push_str(first_and_last[1].to_string().as_str());
                total += string.parse::<u64>().unwrap();
                println!("First and last: {:?}", first_and_last);
                println!();
            } else {
                total += get_total_numeric(line);
            }
        }
        println!("{}", total);
    }

    /// A Number that contains it's alphabetic and numeric values
    #[derive(Clone, Copy, Debug)]
    struct AlphabeticNumber<'a> {
        alphabetic: &'a str,
        numeric: &'a str,
    }

    /// Constant Array of all the Alphabetic Numbers from 1 to 9
    const ALPHABETIC_NUMBERS: [AlphabeticNumber; 9] = [
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

    /// Whether or not the input string contains alphabetic numbers (numbers written in letters)
    fn contains_alphabetic_number(input: &String) -> bool {
        for an in ALPHABETIC_NUMBERS {
            if input.contains(an.alphabetic) {
                return true;
            } else {
                continue;
            }
        }
        return false;
    }

    /// Returns the first and last alphabetic number in the specified input string as an Array
    /// of Strings, containing the numeric value of this number
    fn get_first_and_last_alphabetic_number<'a>(input: &String) -> Vec<&'a str> {
        let mut result: Vec<&str> = vec![];
        let mut first_index: usize = 100000;
        let mut last_index: usize = 0;
        for an in ALPHABETIC_NUMBERS {
            if input.contains(an.alphabetic) {
                let new_index = input.find(an.alphabetic).unwrap(); // unwrap can be called safely, because it's ensured that input contains the alphabetic value
                if new_index < first_index {
                    first_index = new_index;
                    if result.len() > 0 {
                        result[0] = an.numeric;
                    } else {
                        result.push(an.numeric);
                    }
                }
                if new_index >= last_index {
                    last_index = new_index;
                    if result.len() < 2 {
                        result.push(an.numeric);
                        //if result.len() < 2 {
                        //  result.push(an.numeric);
                        //}
                    } else {
                        result[1] = an.numeric;
                    }
                }
                let r_new_index: usize = input.rfind(an.alphabetic).unwrap();
                if r_new_index > last_index {
                    result[1] = an.numeric;
                    last_index = r_new_index;
                }
                //if r_new_index == new_index || r_new_index < last_index { // first case: number only appears once, second case: rfind finds number that is before the last one
                //  continue;
                //} else {
                //}
            } else {
                continue;
            }
        }
        return result;
    }

    /// The different types a number can be encoded in.
    #[derive(PartialEq, Debug)]
    enum NumberType {
        Alphabetic,
        Numeric,
    }

    /// Returns the type of the first and last number in a specified String
    fn get_first_and_last_type(input: &String) -> Vec<NumberType> {
        let mut result: Vec<NumberType> = vec![];
        let mut first_index_numeric: Option<u64> = None;
        let mut last_index_numeric: Option<u64> = None;
        let mut is_first_numeric: bool = true;
        let mut first_index_alphabetic: Option<u64> = None;
        let mut last_index_alphabetic: Option<u64> = None;
        for (index, char) in input.chars().enumerate() {
            if char.is_numeric() {
                println!("For index {}, found number: {}", index, char);
                if is_first_numeric {
                    is_first_numeric = false;
                    first_index_numeric = Some(index as u64);
                }
                last_index_numeric = Some(index as u64);
            }
        }
        for an in ALPHABETIC_NUMBERS {
            let index = input.find(an.alphabetic);
            let r_index = input.rfind(an.alphabetic);
            println!(
                "For an {:?}, found index {:?} and r index: {:?}",
                an, index, r_index
            );
            if index.is_some() {
                if first_index_alphabetic.is_none() {
                    first_index_alphabetic = Some(index.unwrap() as u64); // unwrap on index can be called safely, because index is some
                } else {
                    if index.unwrap() < first_index_alphabetic.unwrap() as usize {
                        first_index_alphabetic = Some(index.unwrap() as u64);
                    }
                }
                if last_index_alphabetic.is_none() {
                    last_index_alphabetic = Some(r_index.unwrap() as u64);
                } else if r_index.unwrap() > last_index_alphabetic.unwrap() as usize {
                    last_index_alphabetic = Some(r_index.unwrap() as u64);
                } else {
                    continue;
                }
            }
        }
        if first_index_alphabetic.is_some() && first_index_numeric.is_some() {
            if first_index_alphabetic < first_index_numeric {
                result.push(NumberType::Alphabetic);
            } else {
                result.push(NumberType::Numeric);
            }
        } else if first_index_alphabetic.is_some() {
            result.push(NumberType::Alphabetic);
        } else {
            result.push(NumberType::Numeric);
        }
        if last_index_numeric.is_some() && last_index_alphabetic.is_some() {
            if last_index_alphabetic > last_index_numeric {
                result.push(NumberType::Alphabetic);
            } else {
                result.push(NumberType::Numeric);
            }
        } else if last_index_alphabetic.is_some() {
            result.push(NumberType::Alphabetic);
        } else {
            result.push(NumberType::Numeric)
        }
        return result;
    }
}
