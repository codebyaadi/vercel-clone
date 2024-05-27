use std::path::Path;
use tokio::process::Command;

pub async fn build_project(build_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = Path::new("downloads").join("output").join(build_id);
    println!(
        "Building project for build_id: {} in {:?} ",
        build_id, project_path
    );

    if !project_path.exists() {
        return Err(format!("Project path doesn't exist: {:?}", project_path).into());
    }

    println!("Installing the dependencies...");
    // Run `npm install`
    let status = Command::new("cmd")
        .arg("/C")
        .arg("npm")
        .arg("install")
        .current_dir(&project_path)
        .status()
        .await?;

    if !status.success() {
        return Err(format!("npm install failed with status: {}", status).into());
    } else {
        println!("Project dependencies installed");
    }

    println!("Building the project...");
    // Run `npm run build`
    let status = Command::new("cmd")
        .arg("/C")
        .arg("npm")
        .arg("run")
        .arg("build")
        .current_dir(&project_path)
        .status()
        .await?;

    if !status.success() {
        return Err(format!("npm run build failed with status: {}", status).into());
    } else {
        println!("Project built successfully!");
    }

    Ok(())
}
