use std::sync::Arc;

trait Subscriber {
    fn on_notify(&self, msg : &str);
}

struct MaleSubScriber;
impl Subscriber for MaleSubScriber {
    fn on_notify(&self, msg : &str) {
        println!(" MaleSubScriber received {msg}");
    }
}

struct FemaleSubScriber;
impl Subscriber for FemaleSubScriber {
    fn on_notify(&self, msg : &str) {
         println!(" MaleSubScriber received {msg}");
    }
}