const CONFIG_JSON: &str = include_str!("../../../config.json");

fn extract_json_str<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    match (|| {
        let needle = format!("\"{}\"", key);
        let after_key = json.split(&needle).nth(1)?;
        let after_colon = after_key.split_once(':')?.1.trim();
        let after_quote = after_colon.strip_prefix('"')?;
        let value = after_quote.split('"').next()?;
        Some(value)
    })() {
        Some(v) => {
            log::info!("extract_json_str: key={}, value={}", key, v);
            Some(v)
        }
        None => {
            log::warn!("extract_json_str: key={}, parse failed", key);
            None
        }
    }
}

pub fn base_dir() -> &'static str {
    let result = extract_json_str(CONFIG_JSON, "baseDir").unwrap_or(".bazel-jdt");
    log::info!("internal_config: base_dir={}", result);
    result
}

pub fn config_file() -> &'static str {
    let result = extract_json_str(CONFIG_JSON, "configFile").unwrap_or(".bazelproject");
    log::info!("internal_config: config_file={}", result);
    result
}

pub fn aspects_dir() -> String {
    let base = base_dir();
    let aspects = extract_json_str(CONFIG_JSON, "aspectsDir").unwrap_or("aspects");
    let result = format!("{}/{}", base, aspects);
    log::info!("internal_config: aspects_dir={}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_reads_from_root_config_json() {
        assert_eq!(base_dir(), ".bazel-jdt");
        assert_eq!(config_file(), ".bazelproject");
        assert_eq!(aspects_dir(), ".bazel-jdt/aspects");
    }
}
