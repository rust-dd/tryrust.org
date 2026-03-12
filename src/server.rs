use dioxus::prelude::*;

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
        tracing::error!("Error creating session: {:?}", e);
        ServerFnError::new("Error creating session")
    })?;

    if !output.status.success() {
        tracing::error!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(ServerFnError::new("Failed to create new project"));
    }

    Ok(session_id)
}

#[server]
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

    let sed_command = if cfg!(target_os = "macos") {
        "gsed"
    } else {
        "sed"
    };
    let modified_code = format!("{};", code);
    let commands = vec![
        format!(
            "{} -i '/^    print/ s/^\\(.*\\)$/\\/\\/\\1/g' {}",
            sed_command, file_path
        ),
        format!(
            "{} -i '/();$/ s/^\\(.*\\)$/\\/\\/\\1/g' {}",
            sed_command, file_path
        ),
        format!(
            "{} -i '$i\\{}' {}",
            sed_command, modified_code, file_path
        ),
        format!("rustfmt -- {}", file_path),
        format!(
            "cargo run --manifest-path ./sessions/{0}/Cargo.toml -- --name tryrust-{0}",
            session_id
        ),
    ];
    let command = commands.join(" && ");

    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd.output().await.map_err(|e| {
        tracing::error!("Error executing command: {:?}", e);
        ServerFnError::new("Error executing command")
    })?;

    if !output.status.success() {
        let _ = fs::copy(&backup_path, &file_path);

        let cargo_stderr = String::from_utf8_lossy(&output.stderr);
        let re = Regex::new(r"error.+").unwrap();
        let errors: Vec<&str> = re
            .find_iter(&cargo_stderr)
            .map(|mat| mat.as_str())
            .collect();

        if !errors.is_empty() {
            return Ok(errors.join("\n"));
        } else {
            return Ok(cargo_stderr.to_string());
        }
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
