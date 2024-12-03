pub mod first {
    use std::fs;

    pub fn first_main() {
        let mut list_one: Vec<i32> = vec![];
        let mut list_two: Vec<i32> = vec![];
        get_input(&mut list_one, &mut list_two);
        first_main_one(list_one.clone(), list_two.clone());
        first_main_two(list_one.clone(), list_two.clone());
    }

    fn get_input(list_one: &mut Vec<i32>, list_two: &mut Vec<i32>) {
        let file_content = fs::read_to_string("../../data/input_first.txt").unwrap();
        for line in file_content.split("\n") {
            if line.is_empty() {
                break;
            }
            let mut splitted = line.split_whitespace();
            let first: i32 = splitted.next().unwrap().parse().unwrap();
            let second: i32 = splitted.next().unwrap().parse().unwrap();
            list_one.push(first);
            list_two.push(second);
        }
    }

    fn first_main_one(mut list_one: Vec<i32>, mut list_two: Vec<i32>) {
        list_one.sort();
        list_two.sort();
        let mut difference: i32 = 0;
        for i in 0..list_one.len() {
            difference += (list_one[i] - list_two[i]).abs();
        }
        println!("Difference: {}", difference);
    }

    fn first_main_two(list_one: Vec<i32>, list_two: Vec<i32>) {
        let mut score: i32 = 0;
        for number in list_one {
            score += number * count_occurences_in_list(list_two.clone(), number);
        }
        println!("Score: {}", score);
    }

    fn count_occurences_in_list(list: Vec<i32>, number_input: i32) -> i32 {
        let mut count: i32 = 0;
        for number in list {
            if number == number_input {
                count += 1;
            } else {
                continue;
            }
        }
        count
    }
}
