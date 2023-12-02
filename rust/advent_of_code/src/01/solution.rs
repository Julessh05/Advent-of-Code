use std::fs;

mod first_december {

fn first() {
    let file_content  = fs::read_to_string("src/01/content.txt").expect("Error");
    let split = file_content.split("\n");
    let mut total : u64 = 0;
    for line in split {
        let mut one : char = 'A';
        let mut two : char = 'B';
        let mut is_first: bool = true;
        for char in line.chars() {
            if char.is_numeric() {
                if is_first {
                    is_first = false;
                    one = char;
                }
                two = char;
            }
        }
        let mut string = one.to_string();
        string.push(two);
        println!("{}", string);
        let int : u64 = string.parse::<u64>().unwrap();
        total += int;
    }
    println!("{}", total);
}
}