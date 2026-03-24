struct FizzBuzzStruct {
    numbers: [i32; 5],
    results: Vec<bool>,
    start: fn(&mut FizzBuzzStruct) -> &mut FizzBuzzStruct,
}

fn fizz_buzz_operation(this: &mut FizzBuzzStruct) -> &mut FizzBuzzStruct {
    this.results.push(true);
    println!("{:?}", this.results);
    return this;
}

fn main() {
    let mut fbs = FizzBuzzStruct {
        numbers: [1,2,3,4,5],
        results: vec![false],
        start: fizz_buzz_operation,
    };

    (fbs.start)(&mut fbs);
}