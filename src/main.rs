use std::io::Write;

fn main() {
    let expected_pizzas = 2;

    println!("You expect {expected_pizzas} pizzas.");
    print!("How many pizzas do you currently have: ");

    let mut raw_actual_pizzas = String::new();

    std::io::stdout()
        .flush()
        .unwrap();

    std::io::stdin()
        .read_line(&mut raw_actual_pizzas)
        .expect("Failed to read input (actual_pizzas)");

    let actual_pizzas = raw_actual_pizzas.trim();

    println!("You have {actual_pizzas} pizzas.");
}
