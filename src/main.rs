use std::io::Write;

fn main() {
    let expected_pizzas:  u8 = 2;
    let expected_burgers: u8 = 4;

    println!("You expect {expected_pizzas} pizzas.");
    println!("You expect {expected_burgers} burgers.");

    let actual_pizzas: u8 = get_actual_food("pizzas");
    let actual_burgers: u8 = get_actual_food("burgers");

    let needed_pizzas: u8 = match expected_pizzas.checked_sub(actual_pizzas) {
        None => 0,
        Some(result) => result,
    };

    let needed_burgers: u8 = match expected_burgers.checked_sub(actual_burgers) {
        None => 0,
        Some(result) => result,
    };

    println!("You need {needed_pizzas} pizzas.");
    println!("You need {needed_burgers} burgers.");
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
