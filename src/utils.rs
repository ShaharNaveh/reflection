use std::fs;
use std::path::PathBuf;

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
    connection_timeout: usize,
    cache_timeout: usize,
    url: &str,
) -> crate::types::MirrorsStatus {
    let cache_path = dbg!(get_cache_file(&url));
    let mirrorstatus = if is_cache_expired(&cache_path, cache_timeout) {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(connection_timeout as u64))
            .build()
            .unwrap();
        let response = client.get(url).send().unwrap();
        let data = response.json().unwrap();

        fs::write(&cache_path, serde_json::to_string_pretty(&data).unwrap()).unwrap();

        data
    } else {
        let cache_file = fs::File::open(&cache_path).unwrap();
        serde_json::from_reader(&cache_file).unwrap()
    };

    mirrorstatus
}
