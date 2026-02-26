use conf::Conf;
use std::path::PathBuf;

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
}

