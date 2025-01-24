use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;
use std::fs;
use std::path::PathBuf;
use crate::error::Result;

pub async fn download_file(url: &str, filepath: &PathBuf, description: &str) -> Result<()> {
    println!("Downloading {}...", description);

    fs::create_dir_all(filepath.parent().unwrap())?;

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let total_size = response.content_length().unwrap_or(0);
    let pb = create_progress_bar(total_size);

    let mut file = fs::File::create(filepath)?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        pb.inc(chunk.len() as u64);
        std::io::copy(&mut chunk.as_ref(), &mut file)?;
    }

    pb.finish_with_message("Download completed");
    Ok(())
}

fn create_progress_bar(total_size: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap());
    pb
}
