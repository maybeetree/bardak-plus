use conf::Conf;
use std::path::PathBuf;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use tokio::sync::OnceCell;

#[derive(Conf)]
pub struct Config {
    #[arg(short, long, env,
        default_value="bardak.sqlite")]
    pub database: String,

    #[arg(long, env,
        default_value="./data/media/upload")]
    pub media_upload_dir: PathBuf,

    #[arg(long, env,
        default_value="./data/media/save")]
    pub media_save_dir: PathBuf,

    #[arg(long, env,
        default_value="./data/media/thumb")]
    pub media_thumb_dir: PathBuf,

    #[arg(long, env)]
    pub thumbspecs: Option<PathBuf>,

}

/// Complicated config that cannot (or should not)
/// be read as a string from env/args and instead
/// is always read from file
pub struct LoadedConfig {
    pub thumbspecs: ThumbSpecs,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThumbSpec {
    pub width: u32,
    pub height: u32,
    pub format: String,
}

macro_rules! spec {
    ($name:expr, $w:expr, $h:expr, $fmt:expr) => {
        ($name.to_string(), ThumbSpec { width: $w, height: $h, format: $fmt.to_string() })
    };
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThumbSpecs {
    pub specs: HashMap<String, ThumbSpec>,
}

impl ThumbSpecs {
    pub fn load(source: &Option<PathBuf>) -> Result<Self> {
        match source {
            None => {
                Ok(Self {
                    specs: HashMap::from([
                            spec!("small-jpg", 160, 90, "jpg"),
                            spec!("small-avif", 160, 90, "avif"),
                            spec!("small-webp", 160, 90, "webp"),
                            spec!("mid-jpg", 640, 360, "jpg"),
                            spec!("mid-avif", 640, 360, "avif"),
                            spec!("mid-webp", 640, 360, "webp"),
                            spec!("big-jpg", 1280, 720, "jpg"),
                            spec!("big-avif", 1280, 720, "avif"),
                            spec!("big-webp", 1280, 720, "webp"),
                    ])
                })
            },
            // TODO nayhow ctx
            Some(path) => {
                Ok(serde_json::from_str(
                        &fs::read_to_string(path)?
                    )?)
            }
        }
    }
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();
static LCONFIG: OnceCell<LoadedConfig> = OnceCell::const_new();

pub async fn get_config() -> &'static Config {
    CONFIG.get_or_init(async || { Config::parse() } ).await
}

pub async fn get_lconfig(config: &Config) -> Result<&'static LoadedConfig> {
    let thumbspecs = ThumbSpecs::load(&config.thumbspecs)?;
    Ok(LCONFIG.get_or_init(async || { LoadedConfig {
            thumbspecs: thumbspecs,
    } } ).await)
}

