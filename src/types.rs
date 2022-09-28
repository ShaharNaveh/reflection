use chrono;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MirrorMetadata {
    pub url: String,
    pub protocol: String,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub completion_pct: f32,
    pub delay: Option<usize>,
    pub duration_avg: Option<f32>,
    pub duration_stddev: Option<f32>,
    pub score: Option<f32>,
    pub active: bool,
    pub country: String,
    pub country_code: String,
    pub isos: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MirrorStatus {
    pub cutoff: usize,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub num_checks: usize,
    pub check_frequency: usize,
    pub urls: Vec<MirrorMetadata>,
    pub version: usize,
}
