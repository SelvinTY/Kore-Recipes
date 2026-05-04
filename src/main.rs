use serde::Deserialize;
use std::path::Path;
use rusqlite::{Connection, params};
use walkdir::WalkDir;
use std::fs;

#[derive(Debug, Deserialize)]
struct PackageRecipe {
    name: String,
    package_name: String,
    url_template: String,
    description: String,
    terminal: bool,
    formats: Vec<String>,
    screenshot_url: Option<String>,
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    maintainer: String,
    license: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "packages.db";
    if Path::new(db_path).exists() {
        fs::remove_file(db_path)?;
    }
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE packages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            description TEXT NOT NULL,
            category TEXT NOT NULL,
            screenshot_url TEXT,
            install_script TEXT NOT NULL,
            binary_name TEXT,
            origin TEXT NOT NULL
        )",
        [],
    )?;

    let origins = vec!["official", "community"];
    for origin in origins {
        let recipes_dir = format!("recipes/{}", origin);
        if !Path::new(&recipes_dir).exists() {
            continue;
        }
        for entry in WalkDir::new(&recipes_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
                let path = entry.path();
                
                let components: Vec<_> = path.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
                let origin_idx = components.iter().position(|c| c == origin).unwrap_or(1);
                
                if components.len() >= origin_idx + 3 {
                    let category = &components[origin_idx + 1];

                    let content = fs::read_to_string(path)?;
                    let recipe: PackageRecipe = toml::from_str(&content)?;

                    println!("Inserting {} into category {} (origin: {})", recipe.package_name, category, origin);

                    let version = "latest".to_string();
                    let screenshot_url = recipe.screenshot_url.clone();
                    let install_script = recipe.url_template.clone();
                    let binary_name = Some(recipe.package_name.clone());

                    conn.execute(
                        "INSERT INTO packages (name, version, description, category, screenshot_url, install_script, binary_name, origin)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![
                            recipe.name,
                            version,
                            recipe.description,
                            category,
                            screenshot_url,
                            install_script,
                            binary_name,
                            origin.to_string()
                        ],
                    )?;
                } else {
                    eprintln!("Warning: file {} does not follow the {{origin}}/{{category}} structure, skipping.", path.display());
                }
            }
        }
    }

    println!("Database built successfully at packages.db");
    Ok(())
}
