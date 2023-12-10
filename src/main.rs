use std::io::Write;

fn main() {
    let expected_pizzas = 2;

    println!("You expect {expected_pizzas} pizzas.");
    print!("How many pizzas do you currently have: ");

    let mut actual_pizzas = String::new();

    std::io::stdout()
        .flush()
        .unwrap();

    std::io::stdin()
        .read_line(&mut actual_pizzas)
        .expect("Failed to read input (actual_pizzas)");

    println!("You have {} pizzas.", actual_pizzas.trim());
}
