use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use tokio::time::{sleep, Duration};

pub struct ChromeDriverService {
    port: u16,
    process: Option<Child>,
    binary_path: PathBuf,
}

impl ChromeDriverService {
    pub async fn start(port: u16) -> Result<Self> {
        let binary_path = Self::ensure_chromedriver().await?;

        info!("Starting ChromeDriver on port {}", port);

        let process = Command::new(&binary_path)
            .arg(format!("--port={}", port))
            .arg("--allowed-ips=")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start chromedriver")?;

        let service = Self {
            port,
            process: Some(process),
            binary_path,
        };

        for i in 0..30 {
            sleep(Duration::from_millis(100)).await;
            if service.is_ready().await {
                info!("ChromeDriver ready on port {}", port);
                return Ok(service);
            }
            debug!("Waiting for ChromeDriver... attempt {}/30", i + 1);
        }

        warn!("ChromeDriver may not be fully ready");
        Ok(service)
    }

    async fn is_ready(&self) -> bool {
        let url = format!("http://localhost:{}/status", self.port);
        match reqwest::get(&url).await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    async fn ensure_chromedriver() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("bottest")
            .join("chromedriver");

        std::fs::create_dir_all(&cache_dir)?;

        let browser_version = Self::detect_browser_version().await?;
        let major_version = browser_version
            .split('.')
            .next()
            .unwrap_or("136")
            .to_string();

        info!("Detected browser version: {}", browser_version);

        let chromedriver_path = cache_dir.join(format!("chromedriver-{}", major_version));

        if chromedriver_path.exists() {
            info!("Using cached chromedriver for version {}", major_version);
            return Ok(chromedriver_path);
        }

        info!("Downloading chromedriver for version {}", major_version);
        Self::download_chromedriver(&major_version, &chromedriver_path).await?;

        Ok(chromedriver_path)
    }

    async fn detect_browser_version() -> Result<String> {
        let browsers = [
            ("brave-browser", "--version"),
            ("brave", "--version"),
            ("google-chrome", "--version"),
            ("chromium-browser", "--version"),
            ("chromium", "--version"),
        ];

        for (browser, arg) in browsers {
            if let Ok(output) = Command::new(browser).arg(arg).output() {
                if output.status.success() {
                    let version_str = String::from_utf8_lossy(&output.stdout);
                    if let Some(version) = Self::extract_version(&version_str) {
                        return Ok(version);
                    }
                }
            }
        }

        info!("No browser detected, using default chromedriver version 136");
        Ok("136.0.7103.113".to_string())
    }

    fn extract_version(output: &str) -> Option<String> {
        let re = regex::Regex::new(r"(\d+\.\d+\.\d+\.\d+)").ok()?;
        re.captures(output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    async fn download_chromedriver(major_version: &str, dest: &PathBuf) -> Result<()> {
        let version_url = format!(
            "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_{}",
            major_version
        );

        let full_version = match reqwest::get(&version_url).await {
            Ok(resp) if resp.status().is_success() => resp.text().await?.trim().to_string(),
            _ => {
                warn!(
                    "Could not find exact version for {}, trying known versions",
                    major_version
                );
                Self::get_known_version(major_version)
            }
        };

        info!("Downloading chromedriver version {}", full_version);

        let download_url = format!(
            "https://storage.googleapis.com/chrome-for-testing-public/{}/linux64/chromedriver-linux64.zip",
            full_version
        );

        let tmp_zip = dest.with_extension("zip");

        let response = reqwest::get(&download_url)
            .await
            .context("Failed to download chromedriver")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to download chromedriver: HTTP {}",
                response.status()
            );
        }

        let bytes = response.bytes().await?;
        std::fs::write(&tmp_zip, &bytes)?;

        let output = Command::new("unzip")
            .arg("-o")
            .arg("-d")
            .arg(dest.parent().unwrap())
            .arg(&tmp_zip)
            .output()
            .context("Failed to unzip chromedriver")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to unzip: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let extracted = dest
            .parent()
            .unwrap()
            .join("chromedriver-linux64")
            .join("chromedriver");
        if extracted.exists() {
            if dest.exists() {
                std::fs::remove_file(dest)?;
            }
            std::fs::rename(&extracted, dest)?;
            std::fs::remove_dir_all(dest.parent().unwrap().join("chromedriver-linux64")).ok();
        }

        std::fs::remove_file(&tmp_zip).ok();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(dest)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(dest, perms)?;
        }

        info!("ChromeDriver downloaded to {:?}", dest);
        Ok(())
    }

    fn get_known_version(major: &str) -> String {
        match major {
            "143" => "143.0.7499.0".to_string(),
            "142" => "142.0.7344.0".to_string(),
            "141" => "141.0.7189.0".to_string(),
            "140" => "140.0.7099.0".to_string(),
            "136" => "136.0.7103.113".to_string(),
            "135" => "135.0.7049.84".to_string(),
            "134" => "134.0.6998.165".to_string(),
            "133" => "133.0.6943.141".to_string(),
            "132" => "132.0.6834.83".to_string(),
            "131" => "131.0.6778.204".to_string(),
            "130" => "130.0.6723.116".to_string(),
            _ => "136.0.7103.113".to_string(),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            info!("Stopping ChromeDriver");
            process.kill().ok();
            process.wait().ok();
        }
        Ok(())
    }

    pub fn cleanup(&mut self) {
        if let Some(mut process) = self.process.take() {
            process.kill().ok();
            process.wait().ok();
        }
    }
}

impl Drop for ChromeDriverService {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        let output = "Brave Browser 143.1.87.55 nightly";
        let version = ChromeDriverService::extract_version(output);
        assert!(version.is_none());

        let output = "Google Chrome 136.0.7103.113";
        let version = ChromeDriverService::extract_version(output);
        assert_eq!(version, Some("136.0.7103.113".to_string()));
    }

    #[test]
    fn test_known_versions() {
        assert_eq!(
            ChromeDriverService::get_known_version("136"),
            "136.0.7103.113"
        );
        assert_eq!(
            ChromeDriverService::get_known_version("143"),
            "143.0.7499.0"
        );
    }
}
