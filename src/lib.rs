use serde::{Deserialize, Serialize};
use std::f32;

pub mod simulator {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum State {
        Idle,
        Operating,
        Charging,
        Error(String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Drone {
        pub id: u8,
        pub name: String,
        pub version: String,
        pub state: State,
        pub location: Vec<String>,
        pub target_location: Vec<String>,
    }

    impl Default for Drone {
        fn default() -> Self {
            Drone {
                id: 0,
                name: "Drone".to_owned(),
                version: "0.0.1".to_owned(),
                state: State::Idle,
                location: vec!["52.529797".to_owned(), "13.413094".to_owned()],
                target_location: Vec::new(),
            }
        }
    }

    impl Clone for Drone {
        fn clone(&self) -> Self {
            Drone {
                id: self.id,
                name: self.name.clone(),
                version: self.version.clone(),
                state: self.state.clone(),
                location: self.location.clone(),
                target_location: self.target_location.clone(),
            }
        }
    }

    impl Drone {
        pub fn new() -> Self {
            Drone {
                ..Default::default()
            }
        }

        pub fn new_with_target(target: Vec<String>) -> Self {
            Drone {
                state: State::Operating,
                target_location: target,
                ..Default::default()
            }
        }

        fn set_location(&mut self, latitude: String, longitude: String) {
            self.location = vec![latitude, longitude];
        }

        pub fn new_target(&mut self, latitude: String, longitude: String) {
            self.target_location = vec![latitude, longitude];
        }

        pub fn state(&self) -> &State {
            &self.state
        }

        fn approach(&mut self) {
            let own_lat = self.location[0].parse::<f32>().unwrap();
            let own_long = self.location[1].parse::<f32>().unwrap();
            let target_lat = self.target_location[0].parse::<f32>().unwrap();
            let target_long = self.target_location[1].parse::<f32>().unwrap();

            let v1 = target_lat - own_lat;
            let v2 = target_long - own_long;

            let magnitude = f32::sqrt(v1.powf(2.0) + v2.powf(2.0));

            if magnitude < 0.001 {
                self.state = State::Idle;
                return;
            }

            let normalized = vec![v1 / magnitude, v2 / magnitude];

            let mut max_velocity = 0.0007;

            if magnitude < 0.002 {
                max_velocity = magnitude / 4.0;
            }

            let vel1 = max_velocity * normalized[0];
            let vel2 = max_velocity * normalized[1];

            let lat = (own_lat + vel1).to_string();
            let long = (own_long + vel2).to_string();

            println!(
                "distance to target ({:?}): {:.4} | normal1: {} normal2: {} vel1: {} vel2: {}",
                self.target_location, magnitude, normalized[0], normalized[1], vel1, vel2
            );

            self.set_location(lat, long);
        }

        pub fn update(&mut self) {
            self.approach();
        }
    }
}
