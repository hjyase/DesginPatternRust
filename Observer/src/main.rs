mod observer;

use std::sync::Arc;

use crate::observer::FemaleSubScriber;
use crate::observer::MaleSubScriber;
use crate::observer::Publisher;

fn main() {
    println!("Hello, observer!");

    let male = Arc::new(MaleSubScriber);
    let femal = Arc::new(FemaleSubScriber);

    let publisher = Publisher::default();
    publisher.register_listener(male.clone());
    publisher.register_listener(femal.clone());

    publisher.notify_message("first notify");

    publisher.unregister_listener(male.clone());
    publisher.notify_message("sencond notify");

    publisher.unregister_listener(femal.clone());
    publisher.notify_message("thirsth notify");
}
