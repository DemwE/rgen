use clap::{command, Parser};

#[derive(Debug, Default, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(
help_template = "{name} {version} {author-section} {about-with-newline} \n {all-args}"
)]
pub struct RgenArgs {
    /// Password length
    #[clap(default_value = "16")]
    #[clap(short, long)]
    pub length: Option<i32>,
    /// Password count
    #[clap(default_value = "1")]
    #[clap(short, long)]
    pub count: Option<i32>,
    /// Use special characters
    #[clap(short, long)]
    pub special: bool,
    /// Raw output
    #[clap(short, long)]
    pub raw: bool,
    /// Save to file
    #[clap(short = 'S', long = "Save")]
    pub save: Option<String>,
    /// Passphrase password
    #[clap(short = 'P', long)]
    pub passphrase: bool,
    /// Passphrase count
    #[clap(default_value = "3")]
    #[clap(short, long)]
    pub passphrase_count: Option<i32>,
}
