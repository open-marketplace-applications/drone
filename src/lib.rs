use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Drone {
    pub id: u8,
    pub name: String,
    pub version: String,
    pub location: Vec<String>,
}

impl Default for Drone {
    fn default() -> Self {
        Drone {
            id: 0,
            name: "Drone".to_owned(),
            version: "0.0.1".to_owned(),
            location: vec!["52.529797".to_owned(), "13.413094".to_owned()],
        }
    }
}

impl Drone {
    pub fn new() -> Self {
        Drone {
            ..Default::default()
        }
    }

    pub fn new_at(latitude: String, longitude: String) -> Self {
        Drone {
            location: vec![latitude, longitude],
            ..Default::default()
        }
    }

    pub fn set_location(&mut self, latitude: String, longitude: String) {
        self.location = vec![latitude, longitude];
    }
}
