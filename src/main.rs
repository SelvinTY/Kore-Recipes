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
            package_name TEXT NOT NULL UNIQUE,
            url_template TEXT NOT NULL,
            description TEXT NOT NULL,
            terminal BOOLEAN NOT NULL,
            maintainer TEXT NOT NULL,
            license TEXT NOT NULL,
            origin TEXT NOT NULL,
            category TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE sources (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            package_id INTEGER NOT NULL,
            format TEXT NOT NULL,
            FOREIGN KEY(package_id) REFERENCES packages(id)
        )",
        [],
    )?;

    let recipes_dir = "recipes";
    for entry in WalkDir::new(recipes_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
            let path = entry.path();
            
            let components: Vec<_> = path.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
            let recipes_idx = components.iter().position(|c| c == "recipes").unwrap_or(0);
            
            if components.len() >= recipes_idx + 3 {
                let origin = &components[recipes_idx + 1];
                let category = &components[recipes_idx + 2];

                let content = fs::read_to_string(path)?;
                let recipe: PackageRecipe = toml::from_str(&content)?;

                println!("Inserting {} (origin: {}, category: {})", recipe.package_name, origin, category);

                conn.execute(
                    "INSERT INTO packages (name, package_name, url_template, description, terminal, maintainer, license, origin, category)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![
                        recipe.name,
                        recipe.package_name,
                        recipe.url_template,
                        recipe.description,
                        recipe.terminal,
                        recipe.metadata.maintainer,
                        recipe.metadata.license,
                        origin,
                        category
                    ],
                )?;

                let package_id = conn.last_insert_rowid();

                for format in recipe.formats {
                    conn.execute(
                        "INSERT INTO sources (package_id, format) VALUES (?1, ?2)",
                        params![package_id, format],
                    )?;
                }
            } else {
                eprintln!("Warning: file {} does not follow the {{origin}}/{{category}} structure, skipping.", path.display());
            }
        }
    }

    println!("Database built successfully at packages.db");
    Ok(())
}
