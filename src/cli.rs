use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(disable_version_flag = true)]
pub struct Args {
    /// Keep the downloaded file compressed
    #[arg(short, long)]
    pub keep_compressed: bool,

    /// Specify VMware Fusion version
    #[arg(short, long)]
    pub version: Option<String>,
}
