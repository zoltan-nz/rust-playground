fn main() {
    let num = 10;
    let greeting = "Hello, world!";
    greet("World".to_string());

    println!(
        "{} {} plus one is {}!",
        greeting,
        num,
        add_one::add_one(num)
    )
}

fn greet(target: String) {
    println!("Hello, {}!", target);
}
