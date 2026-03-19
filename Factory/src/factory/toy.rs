

pub trait Toy {
    fn run(&self);
}

pub struct Roboot;
pub struct Car;

impl Toy for Roboot {
    fn run(&self) {
        println!("Roboot running......")
    }
}

impl Toy for Car {
    fn run(&self) {
        println!("Car running......")
    }
}

pub enum ToyType {
    Robbot,
    Car,
}