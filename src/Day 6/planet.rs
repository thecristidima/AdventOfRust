#[derive(Debug)]
pub struct Planet {
    name: String,
    pub child_names: Vec<String>
}

impl Planet {
    pub fn new(name: &str, child_name: &str) -> Planet {
        let mut child_names = Vec::<String>::new();
        child_names.push(child_name.to_string());

        Planet {
            name: name.to_string(),
            child_names
        }
    }
}