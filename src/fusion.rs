use std::{path::PathBuf, fs, process::Command};
use regex::Regex;
use crate::{
    error::{Result, FusionError},
    utils::{self, version_gte},
    downloader
};

#[derive(Debug)]
pub struct VmwareFusion {
    base_url: String,
    version: String,
    build: String,
    download_dir: PathBuf,
}

impl VmwareFusion {
    pub async fn new(user_version: Option<String>) -> Result<Self> {
        let base_url = "https://softwareupdate.vmware.com/cds/vmw-desktop/fusion".to_string();
        let download_dir = dirs::home_dir()
            .ok_or_else(|| FusionError::System("Home directory not found".to_string()))?
            .join("Downloads");

        let version = match user_version {
            Some(v) => Self::validate_version(&base_url, &v).await?,
            None => Self::get_latest_version(&base_url).await?,
        };

        let build = Self::get_build(&base_url, &version).await?;

        Ok(Self {
            base_url,
            version,
            build,
            download_dir,
        })
    }

    async fn get_latest_version(base_url: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let response = client.get(base_url)
            .send()
            .await?
            .text()
            .await?;

        let re = Regex::new(r#"href="([0-9]+\.[0-9]+\.[0-9]+)/""#)?;
        let versions: Vec<String> = re.captures_iter(&response)
            .map(|cap| cap[1].to_string())
            .collect();

        versions.iter()
            .max_by(|a, b| utils::version_compare(a, b))
            .map(|v| v.to_string())
            .ok_or_else(|| FusionError::Version("No versions found".to_string()))
    }

    async fn validate_version(base_url: &str, version: &str) -> Result<String> {
        let min_version = "13.0.0";
        if !version_gte(version, min_version) {
            return Err(FusionError::Version(
                format!("Version {} is not supported. Fusion 13.0.0 or higher is required.", version)
            ));
        }

        let url = format!("{}/{}/", base_url, version);
        let response = reqwest::get(&url).await;
        match response {
            Ok(_) => Ok(version.to_string()),
            Err(_) => Err(FusionError::Version(format!("Version {} does not exist", version))),
        }
    }

    async fn get_build(base_url: &str, version: &str) -> Result<String> {
        let url = format!("{}/{}/", base_url, version);
        let response = reqwest::get(&url)
            .await?
            .text()
            .await?;

        let re = Regex::new(r#"href="([0-9]+)/""#)?;
        re.captures(&response)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or(FusionError::BuildNotFound)
    }

    pub async fn download(&self) -> Result<PathBuf> {
        let download_url = format!(
            "{}/{}/{}/universal/core/com.vmware.fusion.zip.tar",
            self.base_url, self.version, self.build
        );

        let filename = format!("com.vmware.fusion-{}-{}.zip.tar", self.version, self.build);
        let filepath = self.download_dir.join(&filename);

        downloader::download_file(
            &download_url,
            &filepath,
            &format!("VMware Fusion v{} ({})", self.version, self.build)
        ).await?;

        Ok(filepath)
    }

    pub fn extract(&self, filepath: &PathBuf) -> Result<PathBuf> {
        let extract_dir = filepath.with_extension("");
        fs::create_dir_all(&extract_dir)?;

        println!("Extracting tar...");
        Command::new("tar")
            .args(&["-xf", filepath.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
            .output()?;

        println!("Extracting zip...");
        Command::new("unzip")
            .args(&[
                "-q",
                &format!("{}/com.vmware.fusion.zip", extract_dir.to_str().unwrap()),
                "payload/VMware Fusion.app/*",
                "-d",
                extract_dir.to_str().unwrap(),
            ])
            .output()?;

        let app_path = extract_dir.join("payload").join("VMware Fusion.app");

        // Remove quarantine
        Command::new("xattr")
            .args(&["-dr", "com.apple.quarantine", app_path.to_str().unwrap()])
            .output()?;

        // Cleanup
        self.cleanup(&extract_dir, filepath)?;

        Ok(app_path)
    }

    fn cleanup(&self, extract_dir: &PathBuf, filepath: &PathBuf) -> Result<()> {
        fs::remove_file(filepath)?;
        fs::remove_file(extract_dir.join("com.vmware.fusion.zip"))?;
        fs::remove_file(extract_dir.join("descriptor.xml"))?;
        Ok(())
    }
}
