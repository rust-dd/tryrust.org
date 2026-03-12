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

    let _ = fs::copy(&file_path, &backup_path);

    // Modify the source file using Rust I/O (avoids shell escaping issues with sed)
    let content = fs::read_to_string(&file_path).map_err(|e| {
        tracing::error!("Error reading file: {:?}", e);
        ServerFnError::new("Error reading source file")
    })?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Comment out print statements and lines ending with ();
    for line in &mut lines {
        if line.starts_with("    print") || line.ends_with("();") {
            *line = format!("//{line}");
        }
    }

    // Insert new code before the closing brace of main()
    let modified_code = format!("    {};", code);
    let insert_pos = lines
        .iter()
        .rposition(|l| l.trim() == "}")
        .unwrap_or(lines.len());
    lines.insert(insert_pos, modified_code);

    let new_content = lines.join("\n") + "\n";
    fs::write(&file_path, &new_content).map_err(|e| {
        tracing::error!("Error writing file: {:?}", e);
        ServerFnError::new("Error writing source file")
    })?;

    // Format and compile
    let command = format!(
        "rustfmt -- {file_path} && cargo run --manifest-path ./sessions/{session_id}/Cargo.toml -- --name tryrust-{session_id}"
    );

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

#[server]
pub async fn run_playground(session_id: String, code: String) -> Result<String, ServerFnError> {
    use regex::Regex;
    use std::fs;
    use std::process::Stdio;
    use tokio::process::Command;

    let file_path = format!("./sessions/{}/src/main.rs", session_id);

    // Write the full code (wrap in main if needed)
    let full_code = if code.contains("fn main") {
        code.clone()
    } else {
        format!("fn main() {{\n{code}\n}}")
    };

    fs::write(&file_path, &full_code).map_err(|e| {
        tracing::error!("Error writing playground file: {:?}", e);
        ServerFnError::new("Error writing source file")
    })?;

    let command = format!(
        "rustfmt -- {file_path} && cargo run --manifest-path ./sessions/{session_id}/Cargo.toml -- --name tryrust-{session_id}"
    );

    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd.output().await.map_err(|e| {
        tracing::error!("Error executing playground: {:?}", e);
        ServerFnError::new("Error executing command")
    })?;

    if !output.status.success() {
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
