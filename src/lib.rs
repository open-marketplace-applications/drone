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

        pub fn set_target(&mut self, latitude: String, longitude: String) {
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

            let dx = target_lat - own_lat;
            let dy = target_long - own_long;

            let magnitude = f32::sqrt(dx.powf(2.0) + dy.powf(2.0));

            if magnitude < 0.001 {
                self.state = State::Idle;
                return;
            }

            let xnormal = dx / magnitude;
            let ynormal = dy / magnitude;

            let max_velocity = if magnitude < 0.002 {
                magnitude / 4.0
            } else {
                0.0007
            };

            let xvel = max_velocity * xnormal;
            let yvel = max_velocity * ynormal;

            let lat = (own_lat + xvel).to_string();
            let long = (own_long + yvel).to_string();

            self.set_location(lat, long);

            println!(
                "distance to target ({:?}): {:.4} | xnormal: {} ynormal: {} xvel: {} yvel: {}",
                self.target_location, magnitude, xnormal, ynormal, xvel, yvel
            );
        }

        pub fn update(&mut self) {
            self.approach();
        }
    }
}
