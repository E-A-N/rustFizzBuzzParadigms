struct TaskData {
    name: String,
}

fn example_task(task: &mut TaskData, nummy: i32) {
    task.name.drain(..);
    task.name.push_str(
        // format allows println! syntax to build strings
        // for usage instead of printing
        &format!("Crazy Dazyyy! {}", nummy)
    );
    println!("Oh shit! {} is a crazy task! {}", task.name, nummy);
}

fn actual_main() {
    let mut task1: TaskData = TaskData{name: String::with_capacity(10)};
    example_task(&mut task1, 5);
    example_task(&mut task1, 9);
}

fn main() {
    actual_main();
}