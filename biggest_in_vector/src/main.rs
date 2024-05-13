fn biggest(vector: &[i32]) -> i32 {
    let mut biggest: i32 = vector[0];
    for i in vector {
        if i > &biggest {
            biggest = *i;
        }
    }
    return biggest;
}

fn main() {
    let vector: Vec<i32> = vec![1, 12, 35, 94, 75];
    let biggest: i32 = biggest(&vector);
    println!("{}", biggest);
}
