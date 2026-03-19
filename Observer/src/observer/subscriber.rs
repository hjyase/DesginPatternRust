use crate::observer::listener::Listener;

#[derive(Debug, Default)]
pub struct MaleSubScriber;
impl Listener for MaleSubScriber {
    fn on_received(&self, msg: &str) {
        println!(" MaleSubScriber received {msg}");
    }
}

#[derive(Debug, Default)]
pub struct FemaleSubScriber;
impl Listener for FemaleSubScriber {
    fn on_received(&self, msg: &str) {
        println!(" MaleSubScriber received {msg}");
    }
}
