pub mod factory;

use crate::factory::factory::Factory;

fn main() {
    println!("Hello, factory!");

    let car = Factory::build_toy(factory::ToyType::Car);
    car.run();

    let robbot = Factory::build_toy(factory::ToyType::Robbot);
    robbot.run();
}
