fn main() {
    let start_value: i32 = 1;
    let end_value: i32 = 100;
    let mut fizbuz_store: String = "".to_string();
    for i in start_value..=end_value {
        if i % 3 == 0 {
            fizbuz_store.push_str("fizz");
        }
        if i % 5 == 0 {
            fizbuz_store.push_str("buzz");
        }

        println!("{}-{}", i, fizbuz_store);
        fizbuz_store = "".to_string();
    }
}