use leptos::{server, ServerFnError};

#[server]
pub async fn compile(session_id: String, code: String) -> Result<String, ServerFnError> {
    use std::process::Stdio;
    use tokio::process::Command;

    let mut cmd = Command::new("sh");
    cmd.args([
        "-c",
        &format!(
            "cargo run --manifest-path ./sessions/{0}/Cargo.toml -- --name tryrust-{0}",
            session_id
        ),
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    let output = cmd.output().await.map_err(|e| {
        tracing::info!("Error compiling session: {:?}", e);
        ServerFnError::new("Error compiling session")
    })?;

    tracing::info!("cmd: {:?}", cmd);

    if !output.status.success() {
        tracing::error!(
            "Command failed with status: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(ServerFnError::ServerError(
            "Failed to compile project".into(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
