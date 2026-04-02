fn main() {
    for mut x in 0..100 {
        x = x + 1;
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz");
        }
        else if x % 3 == 0 {
            println!("Fizz");
        }
        else if x % 5 == 0{
            println!("Buzz");
        }
        else {
            println!("{}", x);
        }
    }
}
