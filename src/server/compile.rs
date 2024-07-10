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

    let sed_command = cfg!(target_os = "macos").then(|| "gsed").unwrap_or("sed");
    let modified_code = format!("{};", code);
    let commands = vec![
        format!("cargo fmt -- {}", file_path),
        format!("{} -i 's/print/\\/\\/print/g' {}", sed_command, file_path),
        format!("{} -i '$i\\{}' {}", sed_command, modified_code, file_path),
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
