use bh_daemon::{
    already_running, cleanup_runtime_files, initialize_runtime_files, log_line, serve, stop_remote,
    DaemonConfig,
};

#[tokio::main]
async fn main() {
    let name = std::env::var("BU_NAME").unwrap_or_else(|_| "default".to_string());
    let mut config = DaemonConfig::new(name);
    config.remote_browser_id = std::env::var("BU_BROWSER_ID")
        .ok()
        .filter(|value| !value.is_empty());
    config.browser_use_api_key = std::env::var("BROWSER_USE_API_KEY")
        .ok()
        .filter(|value| !value.is_empty());
    config.remote_file_staging = config.remote_browser_id.is_some()
        || std::env::var("BU_CDP_WS")
            .or_else(|_| std::env::var("BU_CDP_URL"))
            .ok()
            .and_then(|value| remote_cdp_host(&value))
            .map(|host| !is_loopback_host(&host))
            .unwrap_or(false);
    let paths = config.paths();

    if already_running(&config) {
        eprintln!("daemon already running on {}", paths.sock.display());
        return;
    }

    if let Err(err) = initialize_runtime_files(&config) {
        eprintln!("{err}");
        std::process::exit(1);
    }

    let result = serve(&config).await;
    if let Err(err) = &result {
        log_line(&config, &format!("fatal: {err}"));
    }

    match stop_remote(&config).await {
        Ok(true) => {
            if let Some(remote_browser_id) = config.remote_browser_id.as_deref() {
                log_line(
                    &config,
                    &format!("stopped remote browser {remote_browser_id}"),
                );
            }
        }
        Ok(false) => {}
        Err(err) => {
            if let Some(remote_browser_id) = config.remote_browser_id.as_deref() {
                log_line(
                    &config,
                    &format!("stop_remote failed ({remote_browser_id}): {err}"),
                );
            } else {
                log_line(&config, &format!("stop_remote failed: {err}"));
            }
        }
    }
    cleanup_runtime_files(&config);

    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn remote_cdp_host(url: &str) -> Option<String> {
    let without_scheme = url.split_once("://").map(|(_, rest)| rest).unwrap_or(url);
    let authority = without_scheme.split('/').next().unwrap_or(without_scheme);
    if let Some(rest) = authority.strip_prefix('[') {
        return rest.split_once(']').map(|(host, _)| host.to_string());
    }
    Some(
        authority
            .rsplit_once(':')
            .map_or(authority, |(host, _)| host)
            .to_string(),
    )
    .filter(|host| !host.is_empty())
}

fn is_loopback_host(host: &str) -> bool {
    let host = host.trim_matches(['[', ']']).to_ascii_lowercase();
    matches!(host.as_str(), "localhost" | "127.0.0.1" | "::1")
}
