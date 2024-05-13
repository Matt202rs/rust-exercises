fn count(num: i32){
    for i in 1..num+1 {
        println!("{}",i);
    }
}

fn count_down(num: i32) {
    let mut count = num;
    while count > 0 {
        println!("{}",count);
        count -= 1;
    }
}

fn main() {
    count(10);
    count_down(10);
}
