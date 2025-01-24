mod cli;
mod downloader;
mod error;
mod fusion;
mod utils;

use std::fs;
use clap::Parser;
use cli::Args;
use error::Result;
use fusion::VmwareFusion;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    utils::check_macos_version()?;

    let vmware = VmwareFusion::new(args.version).await?;
    let download_path = vmware.download().await?;

    if args.keep_compressed {
        println!("Finished. Downloaded file location: {}", download_path.display());
        return Ok(());
    }

    let app_path = vmware.extract(&download_path)?;

    check_license_files()?;

    println!("\nFinished. You can now move the app into the desired location.");
    println!("VMware Fusion.app location: {}", app_path.display());

    Ok(())
}

fn check_license_files() -> Result<()> {
    if let Ok(entries) = fs::read_dir("/Library/Preferences/VMware Fusion") {
        let license_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name()
                    .to_str()
                    .map(|s| s.starts_with("license-fusion"))
                    .unwrap_or(false)
            })
            .collect();

        if !license_files.is_empty() {
            println!("\nNotice: Existing license file(s) found.");
            println!("Deletion is required if converting to \"Free for Personal Use\" model.");
            println!("To remove:");
            for file in license_files {
                println!("sudo rm \"{}\"", file.path().display());
            }
        }
    }
    Ok(())
}
