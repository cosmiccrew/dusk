pub mod prelude {
    pub use std::f32::consts::*;

    pub use anyhow::{anyhow, bail, ensure, Result};

    pub use crate::{MODELS, MODELS_DIR};
}

pub const MODELS: &str = "spacekit/models/";
pub const MODELS_DIR: &str = concat!("./assets/", "spacekit/models/");
