#![doc = include_str!("../README.md")]

mod file;
pub use file::File;

use core::fmt;
use serde::Deserialize;
use std::{env, path::PathBuf};

const ENV_PROFILE: &str = "APP_PROFILE";
fn env_profile() -> String {
    env::var(ENV_PROFILE).unwrap_or_else(|_| "debug".into())
}

/// The configuration trait.
pub trait Config: fmt::Debug + for<'a> Deserialize<'a> {
    /// The default configuration directory.
    const DEFAULT_CONFIG_DIR: &'static str;
    /// The environment variable prefix.
    const ENV_PREFIX: &'static str;
}

#[derive(Default, Debug)]
pub struct Loader {
    pub explicit_config_dir_path: Option<PathBuf>,
    pub explicit_profile: Option<String>,
}

impl Loader {
    /// Loads the configuration.
    pub fn load<C: Config>(self) -> C {
        let explicit_config_dir_path_env = env::var(format!("{}__CONFIG_DIR", C::ENV_PREFIX)).ok();

        let explicit_config_dir_path = match (
            explicit_config_dir_path_env,
            self.explicit_config_dir_path.as_ref(),
        ) {
            (Some(env), _) => Some(PathBuf::from(env)),
            (None, Some(path)) => Some(PathBuf::from(path)),
            (None, None) => None,
        };

        let config_dir_path =
            explicit_config_dir_path.unwrap_or_else(|| PathBuf::from(C::DEFAULT_CONFIG_DIR));
        let profile = self.explicit_profile.unwrap_or_else(env_profile);

        let common_file = config::File::with_name(&format!("{}/common", config_dir_path.display()))
            .required(false);
        let profile_file =
            config::File::with_name(&format!("{}/{}", config_dir_path.display(), profile))
                .required(false);

        let config = config::Config::builder()
            .add_source(common_file)
            .add_source(profile_file)
            .add_source(config::Environment::with_prefix(C::ENV_PREFIX).separator("__"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();

        tracing::info!(message = "using config", ?config);

        config
    }
}
