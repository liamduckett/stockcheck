use std::io::Write;

fn main() {
    let expected_pizzas = 2;

    println!("You expect {expected_pizzas} pizzas.");

    let actual_pizzas = loop {
        print!("How many pizzas do you currently have: ");

        std::io::stdout()
          .flush()
          .unwrap();

        let mut raw_actual_pizzas = String::new();

        std::io::stdin()
          .read_line(&mut raw_actual_pizzas)
          .expect("Failed to read input (actual_pizzas)");

        match raw_actual_pizzas.trim().parse::<u8>() {
            Err(_) => println!("Invalid amount."),
            Ok(data) => break data,
        };
    };

    println!("You have {actual_pizzas} pizzas.");
}
