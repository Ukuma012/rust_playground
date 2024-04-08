fn main() {
    let marks = vec![10, 9, 8, 4, 6];
    let mut sum = 0;
    for mark in &marks {
        sum = sum + mark;
    }
    println!("Sum of all marks: {:?} is {}", marks, sum);
}
