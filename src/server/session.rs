use leptos::{server, ServerFnError};

#[server]
pub async fn create_session() -> Result<String, ServerFnError> {
    use std::process::Stdio;
    use tokio::process::Command;

    let now = chrono::Utc::now();
    let session_id = now.timestamp_millis().to_string();
    let mut cmd = Command::new("sh");
    cmd.args([
        "-c", 
        &format!(
            "cargo new ./sessions/{0} --bin --name tryrust-{0} && echo \"[workspace]\" >> ./sessions/{0}/Cargo.toml",
            session_id
        ),
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    let output = cmd.output().await.map_err(|e| {
        tracing::info!("Error creating session: {:?}", e);
        ServerFnError::new("Error creating session")
    })?;

    tracing::info!("cmd: {:?}", cmd);

    if !output.status.success() {
        tracing::error!(
            "Command failed with status: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(ServerFnError::ServerError(
            "Failed to create new project".into(),
        ));
    }

    Ok(session_id)
}
