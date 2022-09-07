mod config;

use anyhow::anyhow;
pub use config::*;
use std::path::Path;

#[allow(dead_code)]
pub fn load_config(path: impl AsRef<Path>) -> anyhow::Result<Config> {
    match wd_run::load_config(path) {
        Err(e) => return Err(anyhow!(e)),
        Ok(o) => Ok(o),
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_load_config() {
        match super::load_config("./src/conf/config.toml") {
            Ok(ref o) => {
                let result = serde_json::to_string(o);
                println!("config->{}", result.unwrap_or(String::from("none")));
            }
            Err(e) => {
                println!("load config failed :{}", e.to_string());
            }
        }
    }
    #[test]
    fn test_format() {
        let x = stringify!("{}:{}", "a", 1);
        let s = format!("{}:{}", "a", 1);
        println!("{}---{}", s, x);
    }
}
