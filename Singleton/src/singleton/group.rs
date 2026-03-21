use std::sync::RwLock;


#[derive(Debug)]
pub struct Group {
    name: RwLock<String>,
}

impl Group {
     pub fn new(group_name: String) -> Self {
        Group {
            name : RwLock::new(group_name),
        }
    }

    pub fn get_group_name(&self) -> String {
        if let Ok(name) = self.name.read() {
            name.clone()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn set_group_name(&self, new_name: String) {
        if let Ok(mut name) = self.name.write() {
            *name = new_name;
        } else {
            eprintln!("Failed to acquire write lock for group name");
        }
    }

}