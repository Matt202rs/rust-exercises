use std::io;

fn convert_string_to_int(string:& String) -> i32 {
    let int: i32 = string.trim().parse::<i32>().unwrap();
    return int;
}

fn is_prime(num: i32) -> bool {
    let mut prime: bool = true;
    let mut count: i32 = 2;
    let limit: i32 = (num as f64).sqrt() as i32+1;

    if num <= 1 {
        prime = false;
    } else {
        while count < limit {
            if num % count == 0 {
                prime = false;
                break;
            }
            count += 1;
        }
    }
    return prime;
}

fn main() {

    println!("Please insert a number to check:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error while reading output!");

    let number: i32 = convert_string_to_int(&input);
    let value: bool = is_prime(number);
    if value == true {
        println!("The number {} is a prime number!", number);
    } else {
        println!("The number {} is not a prime number!", number);
    }
}
