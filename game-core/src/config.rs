use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ally {
    pub follow_distance: f32,
    pub max_distance: f32,
    pub min_distance: f32,
}

impl Default for Ally {
    fn default() -> Self {
        Ally {
            follow_distance: 0.0,
            max_distance: 0.0,
            min_distance: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameoffConfig {
    pub ally: Ally,
    pub speed: f32,
}

impl Default for GameoffConfig {
    fn default() -> Self {
        GameoffConfig {
            speed: 0.0,
            ally: Ally::default(),
        }
    }
}
