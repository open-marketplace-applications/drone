use serde::{Deserialize, Serialize};
use std::f32;

pub mod simulator {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Drone {
        pub id: u8,
        pub name: String,
        pub version: String,
        pub location: Vec<String>,
        pub target_location: Vec<String>,
    }

    impl Default for Drone {
        fn default() -> Self {
            Drone {
                id: 0,
                name: "Drone".to_owned(),
                version: "0.0.1".to_owned(),
                location: vec!["52.529797".to_owned(), "13.413094".to_owned()],
                target_location: vec!["52.529797".to_owned(), "13.413094".to_owned()],
            }
        }
    }

    impl Drone {
        pub fn new() -> Self {
            Drone {
                ..Default::default()
            }
        }

        pub fn new_from_target(target: Vec<String>) -> Self {
            Drone {
                target_location: target,
                ..Default::default()
            }
        }

        pub fn set_location(&mut self, latitude: String, longitude: String) {
            self.location = vec![latitude, longitude];
        }

        pub fn set_target_location(&mut self, latitude: String, longitude: String) {
            self.target_location = vec![latitude, longitude];
        }

        fn approach(&mut self) {
            let own_lat = self.location[0].parse::<f32>().unwrap();
            let own_long = self.location[1].parse::<f32>().unwrap();
            let target_lat = self.target_location[0].parse::<f32>().unwrap();
            let target_long = self.target_location[1].parse::<f32>().unwrap();

            let max_velocity = 0.0005;

            let v1 = target_lat - own_lat;
            let v2 = target_long - own_long;

            let mag = f32::sqrt(v1.powf(2.0) + v2.powf(2.0));
            let normalized = vec![v1 / mag, v2 / mag];

            let vel1 = match normalized[0].abs() {
                n if n > max_velocity => {
                    if normalized[0] > 0.0 {
                        max_velocity
                    } else {
                        -max_velocity
                    }
                }
                _ => normalized[0],
            };

            let vel2 = match normalized[1].abs() {
                n if n > max_velocity => {
                    if normalized[1] > 0.0 {
                        max_velocity
                    } else {
                        -max_velocity
                    }
                }
                _ => normalized[0],
            };

            let lat = (own_lat + vel1).to_string();
            let long = (own_long + vel2).to_string();

            let distance =
                f32::sqrt((target_lat - own_lat).powf(2.0) + (target_long - own_long).powf(2.0));

            println!(
                "distance to target ({:?}): {:.4}",
                self.target_location, distance
            );

            self.set_location(lat, long);
        }

        pub fn update(&mut self) {
            self.approach();
        }

        pub fn get_serialized(&self) -> Self {
            Drone {
                id: self.id,
                name: self.name.clone(),
                version: self.version.clone(),
                location: self.location.clone(),
                target_location: self.target_location.clone(),
            }
        }
    }
}
