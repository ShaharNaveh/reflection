use std::fs;
use std::path::PathBuf;

use crate::types::MirrorsStatus;

fn get_cache_file(url: &str) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap().join(&crate::consts::NAME);
    fs::create_dir_all(&cache_dir).unwrap();
    let name = base64::encode_config(&url, base64::URL_SAFE);
    cache_dir.join(&name).with_extension("json")
}

fn is_cache_expired(cache_path: &PathBuf, cache_timeout: usize) -> bool {
    match fs::metadata(cache_path) {
        Ok(metadata) => {
            dbg!(metadata.modified().unwrap().elapsed().unwrap().as_secs() > cache_timeout as u64)
        }
        Err(_) => return true,
    }
}

pub fn get_mirrorstatus(
    _connection_timeout: usize,
    cache_timeout: usize,
    url: &str,
) -> MirrorsStatus {
    let cache_path = dbg!(get_cache_file(&url));
    let mirrorstatus: MirrorsStatus = if is_cache_expired(&cache_path, cache_timeout) {
        let response = reqwest::blocking::get(url).unwrap();
        let data = response.json().unwrap();

        fs::write(&cache_path, serde_json::to_string_pretty(&data).unwrap()).unwrap();

        data
    } else {
        let cache_file = fs::File::open(&cache_path).unwrap();
        serde_json::from_reader(&cache_file).unwrap()
    };

    mirrorstatus
}
