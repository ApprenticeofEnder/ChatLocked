use std::num::NonZeroU32;

pub struct AppState {
    pub keygen_config: KeygenConfig,
}

pub struct KeygenConfig {
    pub pbkdf2_iterations: NonZeroU32,
}
