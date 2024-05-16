use std::io;

fn convert_string_to_int(string:& String) -> i32 {
    let int: i32 = string.trim().parse::<i32>().unwrap();
    return int;
}

fn multiplication_table(number: i32) {
    println!("Here's the multiplication table for the number {}:", number);
    for i in 1..11 {
        println!("{} x {} = {}", number, i, number*i);
    }
}

fn main() {

    let mut input: String = String::new();
    println!("Please insert a number to show the multiplication table:");
    io::stdin().read_line(&mut input).expect("Error while reading input number!");

    let number: i32 = convert_string_to_int(&input);
    multiplication_table(number);

}