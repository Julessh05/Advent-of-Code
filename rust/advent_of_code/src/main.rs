mod twenty_three;

fn main() {}

mod solution {
    use std::fs;

    fn get_content() -> Vec<String> {
        let content: String = fs::read_to_string("twenty_three/days/second/content.txt")
            .expect("Error reading content of second day");
        let mut vec: Vec<String> = content.split("\n").map(str::to_string).collect();
        vec.retain(|line| !line.is_empty());
        let mut result: Vec<String> = vec![];
        for line in vec {
            result.push(line);
        }
        return vec;
    }

    pub fn first() {
        // Allowed: 12 red cubes, 13 green cubes, and 14 blue cubes
        let content = get_content();
    }
}
