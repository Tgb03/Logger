use reqwest::blocking::get;
use semver::Version;

pub fn get_latest_version() -> Option<String> {
    let response = get("https://raw.githubusercontent.com/Tgb03/Logger/master/Cargo.toml");
    // println!("{:?}", response);
    let response = response.ok()?.text().ok()?;

    let parsed: toml::Value = toml::from_str(&response).ok()?;

    // println!("Parsed: {:?}", parsed);

    parsed
        .get("workspace")?
        .get("package")?
        .get("version")?
        .as_str()
        .map(|s| s.to_string())
}

pub fn is_there_new_version(latest_version: &String) -> Option<bool> {
    let current = env!("CARGO_PKG_VERSION");

    println!(
        "Version camparison: latest: {} current: {}",
        latest_version, current
    );

    let current = Version::parse(current).ok()?;
    let latest = Version::parse(latest_version.as_str()).ok()?;

    Some(latest > current)
}
