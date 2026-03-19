
use crate::factory::{Car, Roboot, toy::{Toy, ToyType}};


pub struct Factory;

impl Factory {
    pub fn build_toy(toy_type: ToyType) -> Box<dyn Toy> {
        match toy_type {
            ToyType::Car => Box::new(Car),
            ToyType::Robbot => Box::new(Roboot),
        }
    }
}