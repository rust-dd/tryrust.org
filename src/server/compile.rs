use leptos::{server, ServerFnError};

#[server(endpoint = "/compile")]
pub async fn compile(session_id: String, code: String) -> Result<String, ServerFnError> {
    use regex::Regex;
    use std::fs;
    use std::process::Stdio;
    use tokio::process::Command;

    let file_path = format!("./sessions/{}/src/main.rs", session_id);
    let backup_path = format!("./sessions/{}/src/main.rs.bak", session_id);

    let _ = fs::copy(&file_path, &backup_path).map_err(|e| {
        tracing::error!("Error creating backup: {:?}", e);
        ServerFnError::new("Error creating backup")
    });

    let mut command = String::new();

    if cfg!(target_os = "linux") {
        command = format!(
            "sed -i '$i\\{0}' {1} && cargo run --manifest-path ./sessions/{2}/Cargo.toml -- --name tryrust-{2}",
            code, file_path, session_id
        );
    } else if cfg!(target_os = "macos") {
        command = format!(
            "gsed -i '$i\\{0}' {1} && cargo run --manifest-path ./sessions/{2}/Cargo.toml -- --name tryrust-{2}",
            code, file_path, session_id
        );
    }

    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd.output().await.map_err(|e| {
        tracing::error!("Error executing command: {:?}", e);
        ServerFnError::new("Error executing command")
    })?;

    tracing::info!("cmd: {:?}", cmd);
    if !output.status.success() {
        let _ = fs::copy(&backup_path, &file_path).map_err(|e| {
            tracing::error!("Error creating backup: {:?}", e);
            ServerFnError::new("Error creating backup")
        });

        let cargo_stderr = String::from_utf8_lossy(&output.stderr);
        let re = Regex::new(r"error.+").unwrap();
        let errors: Vec<&str> = re
            .find_iter(&cargo_stderr)
            .map(|mat| mat.as_str())
            .collect();
        tracing::info!("Cargo error: {}", cargo_stderr);
        if !errors.is_empty() {
            let error_messages = errors.join("\n");
            tracing::info!("Command failed with errors: {}", error_messages);
            return Ok(error_messages);
        } else {
            tracing::info!("Command failed with unrecognized errors: {}", cargo_stderr);
            return Ok(cargo_stderr.to_string());
        }
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
