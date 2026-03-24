struct TaskData {
    name: String,
}

fn example_task(task: &mut TaskData, nummy: i32) {
    println!("Oh shit! {} is a crazy task! {}", task.name, nummy);
}

fn main() {
    let mut task1: TaskData = TaskData{name: String::with_capacity(10)};
    task1.name.drain(..);
    task1.name.push_str("Aye!!");
    example_task(&mut task1, 5);
    example_task(&mut task1, 9);
}