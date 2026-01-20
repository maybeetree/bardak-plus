use conf::Conf;

#[derive(Conf)]
pub struct Config {
    #[arg(short, long, env,
        default_value="bardak.sqlite")]
    pub database: String,
}

