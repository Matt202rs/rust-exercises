fn biggest(vector: &[i32]) -> i32 {
    let mut biggest = vector[0];
    for i in vector {
        if i > biggest {
            biggest = i;
        }
    }
    return biggest;
}

fn main() {
    let vector = vec![1, 12, 35, 94, 75];
    let biggest = biggest(vector);
    println!("{}", biggest);
}
