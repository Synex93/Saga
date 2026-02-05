use clap::Subcommand;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
    pub model: Models,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Models {
    Authentication = 0,
    Powershell = 1,
}

impl Config {
    pub fn new(path: String, model: Models) -> Self {
        Self { path, model }
    }
}
