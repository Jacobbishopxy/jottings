//! Error Handling

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

use serde::Deserialize;
use serde_json::from_str;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

/*
1. anyhow::Result + anyhow::Context
*/

#[allow(dead_code)]
fn get_cluster_info(path: &str) -> Result<ClusterMap> {
    let config =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read from {path}"))?;
    let map = from_str(&config);

    match map {
        Ok(map) => Ok(map),
        Err(e) => Err(anyhow!("Failed to parse config: {e}")),
    }
}

#[test]
fn test_get_cluster_info() {
    let cm = get_cluster_info("./mock/cluster_map.json");
    println!("{cm:?}");
}

/*
2. thiserror::Error

Note: `anyhow::Error` is the superset of `ClusterMapError`. In other words,
the type conversion from any type who has implemented `thiserror::Error` to
`anyhow::Error` is given.
*/

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ClusterMapError {
    #[error("Invalid range of range (expected in 0-100), got {0}")]
    InvalidGroup(i32),
}

#[allow(dead_code)]
impl ClusterMap {
    fn validate(self) -> Result<Self> {
        if self.group < 0 || self.group > 100 {
            Err(ClusterMapError::InvalidGroup(self.group).into())
        } else {
            Ok(self)
        }
    }
}

#[allow(dead_code)]
fn get_cluster_info_pro(path: &str) -> Result<ClusterMap> {
    let config =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read from {}", path))?;
    let map: ClusterMap = from_str(&config)?;
    let map = map.validate()?;
    Ok(map)
}

#[test]
fn test_get_cluster_info_pro() {
    match get_cluster_info_pro("./mock/cluster_map.json") {
        Ok(cm) => println!("{cm:?}"),
        Err(e) => println!("{e:?}"),
    };
}
