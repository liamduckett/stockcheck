use std::io::Write;

fn main() {
    let expected_pizzas:  u8 = 2;

    println!("You expect {expected_pizzas} pizzas.");

    let actual_pizzas: u8 = get_actual_food("pizzas");

    let needed_pizzas: u8 = match expected_pizzas.checked_sub(actual_pizzas) {
        None => 0,
        Some(result) => result,
    };

    println!("You need {needed_pizzas} pizzas.");
}

fn get_actual_food(food: &str) -> u8 {
    return loop {
       print!("How many {food} do you currently have: ");

       std::io::stdout()
         .flush()
         .unwrap();

       let mut raw_actual_food = String::new();

       std::io::stdin()
         .read_line(&mut raw_actual_food)
         .expect("Failed to read input (actual_pizzas)");

       match raw_actual_food.trim().parse::<u8>() {
           Err(_) => println!("Invalid amount."),
           Ok(data) => break data,
       };
   };
}
