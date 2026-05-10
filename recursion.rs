fn double_five_times(current_value : i32, iteration: i8) -> i32 {
    if iteration >= 5{
        return current_value;
    }
    else {
        return double_five_times(
            current_value * 2,
            iteration + 1
        )
    }
}

fn fizz_buzz(target: i8, iteration: i8) {
    if iteration >= target {
        return
    }

    if iteration % 3 == 0 && iteration % 5 == 0 {
        println!("Fizz Buzz!");
    }
    else if iteration % 3 == 0 {
        println!("Fizz!");
    }
    else if iteration % 5 == 0 {
        println!("Buzz!");
    }
    else {
        println!("{}", iteration);
    }

    return fizz_buzz(target, iteration + 1);
}

fn main() {
    println!("herro robo!");
    println!("{} doubled five times is: {}", 4, double_five_times(4, 0));
    fizz_buzz(100, 0);
}