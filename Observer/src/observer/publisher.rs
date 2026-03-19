use std::sync::Arc;
use std::sync::Mutex;

use crate::observer::listener::Listener;

#[derive(Debug, Default)]
pub struct Publisher {
    listeners : Mutex<Vec<Arc<dyn Listener>>>, 
}

impl Publisher {
    pub fn register_listener(&self, listener : Arc<dyn Listener>) {
        if let Ok(mut ls) = self.listeners.try_lock() {
            ls.push(listener);
            println!("register_listener successed, current length is {}", ls.len());
        } else {
            println!("register_listener failure, current length is {}", self.listeners.try_lock().unwrap().len());
        };

        if let Some(ll) =  self.listeners.try_lock().ok(){
            let size = ll.len();
        } else {

        }
    }

    pub fn unregister_listener(&self, listener : Arc<dyn Listener>) {
        if let Ok(mut ls) = self.listeners.try_lock() {
           ls.retain(|l| !Arc::ptr_eq(l, &listener));
           println!("unregister_listener successed, current length is {}", ls.len());
        } else {
            println!("unregister_listener failure, current length is {}", self.listeners.try_lock().unwrap().len());
        }
    }

    pub fn notify_message(&self, msg : &str) {
        match self.listeners.try_lock() {
            Ok(ls) => {
                println!("notify message for {} lisntener", ls.len());
                for l in ls.iter() {
                    l.on_received(msg);
                }
            }
            Err(r) => {
                println!("notify message failure");
            }
        }
    }
}