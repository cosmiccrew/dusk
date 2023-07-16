pub mod prelude {
    pub use std::f32::consts::*;

    pub use anyhow::{anyhow, bail, ensure, Result};

    pub use crate::{MODELS, MODELS_DIR};
}

pub const MODELS: &str = "spacekit/models/";
pub const MODELS_DIR: &str = {
    if cfg!(all(
        target_os = "macos",
        not(any(debug_assertions, features = "dynamic_linking"))
    )) {
        //when LazyLock is stabilised, use it here in combination with std::env::current_exe()
        "../Resources/assets/spacekit/models/"
    } else {
        "assets/spacekit/models/"
    }
};
