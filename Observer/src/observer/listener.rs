pub trait Listener: Send + Sync + std::fmt::Debug {
    fn on_received(&self, msg: &str);
}
