use std::io::Write;

fn main() {
    let mut pizza = build_food("pizzas", 2);

    println!("You expect {} {}.", pizza.expected, pizza.name);

    pizza = pizza.get_actual()
                 .get_needed();

    println!("You need {} {}.", pizza.needed, pizza.name);
}

struct Food {
    name: String,
    expected: u8,
    actual: u8,
    needed: u8,
}

impl Food {
    fn get_actual(self) -> Food {
        return Food {
            actual: get_actual_food(&self.name),
            ..self
        };
    }

    fn get_needed(self) -> Food {
        return Food {
            needed: get_needed_food(self.expected, self.actual),
            ..self
        }
    }
}

fn build_food(name: &str, expected: u8) -> Food {
    return Food {
        name: String::from(name),
        expected: expected,
        actual: 0,
        needed: 0,
    }
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

fn get_needed_food(expected_food: u8, actual_food: u8) -> u8 {
    return match expected_food.checked_sub(actual_food) {
        None => 0,
        Some(result) => result,
    };
}
