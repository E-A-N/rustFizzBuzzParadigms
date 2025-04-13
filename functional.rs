
fn main() {
    let fizz_buzz: Vec<String> = (1..100)
        .map(|x| {
            let mut value: String = format!("{}-", x).to_string();
            if x % 3 == 0 {
                value.push_str("Fizz");
            }
            if x % 5 == 0 {
                value.push_str("Buzz");
            }
            
            return value;
        })
        .filter(|x| *x != "".to_string())
        .collect();

    println!("{:?}", fizz_buzz);
}