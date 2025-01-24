use std::process::Command;
use crate::error::{Result, FusionError};

pub fn check_macos_version() -> Result<()> {
    if cfg!(target_os = "macos") {
        let output = Command::new("sw_vers")
            .arg("-productVersion")
            .output()?;
        let version = String::from_utf8(output.stdout)
            .map_err(|e| FusionError::System(e.to_string()))?;

        if version.split('.').next()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0) < 11 {
            println!("Warning: VMware Fusion v13.0.0+ requires macOS 11 or higher.");
        }
    }
    Ok(())
}

pub fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let a_parts: Vec<u32> = a.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    let b_parts: Vec<u32> = b.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    a_parts.cmp(&b_parts)
}

pub fn version_gte(v1: &str, v2: &str) -> bool {
    version_compare(v1, v2) != std::cmp::Ordering::Less
}
