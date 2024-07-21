use open;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use tauri::api::dialog::FileDialogBuilder;
//use tauri::Manager;

const URL_LATEST_BUILD_API:&str = "https://api.github.com/repos/RDPROST/teacher_achivements/releases/latest";
const URL_LATEST_BUILD:&str = "https://github.com/RDPROST/teacher_achivements/releases/latest";


#[derive(Serialize, Deserialize)]
pub struct Release {
	tag_name: String,
	body: String,
	assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
	browser_download_url: String,
	name: String, // Добавляем поле для имени файла
}

#[tauri::command]
pub async fn check_for_updates() -> Result<Release, String> {
	let url = URL_LATEST_BUILD_API;
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header("User-Agent", "teacher_achivements")
		.send()
		.await
		.map_err(|e| e.to_string())?;

	let release: Release = res.json().await.map_err(|e| e.to_string())?;
	Ok(release)
}

#[tauri::command]
pub async fn download_file(url: &str, path: &str) -> Result<(), String> {
	let response = reqwest::Client::new()
		.get(url)
		.header(USER_AGENT, "my-app")
		.send()
		.await
		.map_err(|e| e.to_string())?;

	let mut file = File::create(path).map_err(|e| e.to_string())?;
	let content = response.bytes().await.map_err(|e| e.to_string())?;
	file.write_all(&content).map_err(|e| e.to_string())?;

	open::that(path).map_err(|e| e.to_string())?;

	Ok(())
}

#[tauri::command]
pub async fn open_save_dialog(default_file_name: String) -> Result<String, String> {
	let (tx, rx) = std::sync::mpsc::channel();

	FileDialogBuilder::new()
		.set_title("Save File")
		.set_file_name(&default_file_name)
		.save_file(move |file_path| {
			if let Some(path) = file_path {
				tx.send(path.display().to_string())
					.expect("Failed to send path");
			} else {
				tx.send("".to_string()).expect("Failed to send path");
			}
		});

	let file_path = rx.recv().expect("Failed to receive path");
	if file_path.is_empty() {
		Err("No file selected".to_string())
	} else {
		Ok(file_path)
	}
}

#[tauri::command]
pub async fn get_download_url(release: Release) -> Result<(String, String), String> {
	let os = env::consts::OS;
	let asset_name = match os {
		"windows" => "x64-setup.exe",
		"macos" => "x64.dmg",
		"linux" => "amd64.AppImage",
		_ => return Err("Unsupported OS".to_string()),
	};

	for asset in release.assets {
		if asset.browser_download_url.contains(asset_name) {
			return Ok((asset.browser_download_url, asset.name));
		}
	}

	Err("No suitable asset found".to_string())
}

#[tauri::command]
pub async fn open_link() -> Result<(), String> {
	let link = URL_LATEST_BUILD;
	open::that(link).map_err(|e| e.to_string())?;
	Ok(())
}