use std::io;

fn convert_string_to_int(string:& String) -> i32 {
    let int: i32 = string.trim().parse::<i32>().unwrap();
    return int;
}

fn main() {
    println!("Please insert the number you wish to get the factorial value: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error while reading input!");

    let mut count: i32 = convert_string_to_int(&input);
    let mut factorial: i32 = count;

    while (count - 1) > 1 {
        factorial = factorial * (count - 1);
        count = count - 1;
    }

    println!("The factorial value of {} is: {}", number, factorial);
}
