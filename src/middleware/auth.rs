use crate::config::get_valid_apikeys;

pub fn is_valid_apikey(apikey: &str) -> bool {
    let valid_keys = get_valid_apikeys();
    valid_keys.iter().any(|key| key == apikey)
}
