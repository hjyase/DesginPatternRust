use std::sync::{Arc, OnceLock};

use crate::singleton::group::Group;

#[derive(Debug)]
pub struct Deparkment {
    name: String,
    group: Arc<Group>,
}

impl Deparkment {
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<Deparkment> = OnceLock::new();
        INSTANCE.get_or_init(|| Deparkment {
            name: "Celluar_Communication".to_string(),
            group: Arc::new(Group::new("Platform".to_string())),
        })
    }

    pub fn get_group(&self) -> Arc<Group> {
        self.group.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
