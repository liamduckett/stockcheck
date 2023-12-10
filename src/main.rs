fn main() {
    let expected_pizzas = 2;

    println!("You expect {expected_pizzas} pizzas.");
    println!("How many pizzas do you currently have:");

    let mut actual_pizzas = String::new();

    std::io::stdin()
        .read_line(&mut actual_pizzas)
        .expect("Failed to read input (actual_pizzas)");

    println!("You have {} pizzas.", actual_pizzas.trim());
}
