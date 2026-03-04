use conf::Conf;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Context;
use anyhow::Result;
use anyhow::Error;

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

    #[arg(long, env,
        default_value = "1280x720")]
    pub image_size_large: Resolution,

    #[arg(long, env, 
        default_value = "640x360")]
    pub image_size_medium: Resolution,

    #[arg(long, env,
        default_value = "160x90")]
    pub image_size_small: Resolution,
}

#[derive(Clone, Debug)]
pub struct Resolution(pub u32, pub u32);

impl FromStr for Resolution {
    type Err = Error;  // this tells rust that we want to use anyhow error
                       // as our error type for this struct

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('x').collect();

        anyhow::ensure!(parts.len() == 2, "expected format 'WIDTHxHEIGHT'");

        let width = parts[0].parse::<u32>()
            .context("invalid width")?;
        let height = parts[1].parse::<u32>()
            .context("invalid height")?;

        Ok(Resolution(width, height))
    }
}


