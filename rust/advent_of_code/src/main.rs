mod twenty_three;

fn main() {
    thrid();
}

/// Runs all solutions of the first day
fn _first() {
    twenty_three::days::first::solution::first();
    twenty_three::days::first::solution::second();
    println!();
}

/// Runs all the solutions of the second day
fn _second() {
    twenty_three::days::second::solution::first();
    twenty_three::days::second::solution::second();
    println!();
}

fn thrid() {
    twenty_three::days::third::solution::first();
}
