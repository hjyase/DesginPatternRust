pub mod builder;

use crate::builder::burger_builder::BurgerBuilder;
use crate::builder::burger_builder::BurgerComponent;


fn main() {
    println!("Hello, builder!");

    let builder1 = BurgerBuilder::new();
    builder1.add_component(BurgerComponent::Lettuce)
        .add_component(BurgerComponent::Cheese)
        .add_component(BurgerComponent::TopBun)
        .build();

    let builder2 = BurgerBuilder::new();
    builder2.add_component(BurgerComponent::Cheese)
        .add_component(BurgerComponent::Patty)
        .build();

}
