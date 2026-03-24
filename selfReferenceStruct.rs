struct FizzBuzzStruct {
    amount: i32,
    results: Vec<String>,
    operate: fn(&mut FizzBuzzStruct) -> &mut FizzBuzzStruct,
}

fn fizz_buzz_operation(this: &mut FizzBuzzStruct) -> &mut FizzBuzzStruct {
    for n in 1..=this.amount {
        let mut stringy : String = format!("{} ", n);
        if n % 3 == 0{
            stringy.push_str("fizz");
        }

        if n % 5 == 0{
            stringy.push_str("buzz");
        }
        this.results.push(stringy);
    }
    println!("{:?}", this.results);
    return this;
}

fn main() {
    let mut fbs = FizzBuzzStruct {
        amount: 100,
        results: vec![],
        operate: fizz_buzz_operation,
    };
    (fbs.operate)(&mut fbs);
}