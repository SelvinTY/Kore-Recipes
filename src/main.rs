use serde::Deserialize;
use std::path::Path;
use rusqlite::{Connection, params};
use walkdir::WalkDir;
use std::fs;

#[derive(Debug, Deserialize)]
struct PackageRecipe {
    #[serde(alias = "package_name")]
    id: String,
    name: String,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default = "default_category")]
    category: String,
    #[serde(alias = "url_template")]
    install_script: String,
    binary_name: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    screenshots: Option<Vec<String>>,
}

fn default_version() -> String { "latest".to_string() }
fn default_category() -> String { "Utility".to_string() }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "packages.db";
    if Path::new(db_path).exists() {
        fs::remove_file(db_path)?;
    }
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS packages (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            description TEXT,
            category TEXT NOT NULL,
            icon_url TEXT,
            screenshots TEXT,
            install_script TEXT NOT NULL,
            binary_name TEXT,
            origin TEXT NOT NULL
        )",
        [],
    )?;

    let recipes_dir = "recipes";
    if !Path::new(recipes_dir).exists() {
        return Ok(());
    }

    for entry in WalkDir::new(recipes_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
            let path = entry.path();
            
            let components: Vec<_> = path.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
            let recipes_idx = components.iter().position(|c| c == "recipes").unwrap_or(0);
            
            let origin = if components.len() > recipes_idx + 1 {
                components[recipes_idx + 1].clone()
            } else {
                "unknown".to_string()
            };

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading {}: {}", path.display(), e);
                    continue;
                }
            };

            let recipe: PackageRecipe = match toml::from_str(&content) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error parsing {}: {}", path.display(), e);
                    continue;
                }
            };

            let screenshots_json = match &recipe.screenshots {
                Some(s) => Some(serde_json::to_string(s).unwrap_or_else(|_| "[]".to_string())),
                None => None,
            };

            let category = if recipe.category == "Utility" && components.len() >= recipes_idx + 3 {
                components[recipes_idx + 2].clone()
            } else {
                recipe.category.clone()
            };

            conn.execute(
                "INSERT OR REPLACE INTO packages (id, name, version, description, category, icon_url, screenshots, install_script, binary_name, origin)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    recipe.id,
                    recipe.name,
                    recipe.version,
                    recipe.description.unwrap_or_default(),
                    category,
                    recipe.icon_url,
                    screenshots_json,
                    recipe.install_script,
                    recipe.binary_name.unwrap_or_else(|| recipe.id.clone()),
                    origin
                ],
            )?;
        }
    }

    Ok(())
}
