use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const KIBBLE_FILE: &str = "Kibble.toml";
const KIBBLE_LOCK: &str = "Kibble.lock";
const LITTER_BOX: &str = "litter_box"; // target directory

/// Initialize a new LolRust project with a Kibble.toml
pub fn init(name: Option<&str>) -> Result<(), String> {
    let project_name = name.unwrap_or("my_kitteh_project");
    let project_dir = Path::new(project_name);

    if project_dir.exists() {
        return Err(format!("'{}' already exists! pick another name kitteh", project_name));
    }

    fs::create_dir_all(project_dir.join("src"))
        .map_err(|e| format!("couldnt make directories: {}", e))?;

    let kibble_toml = format!(
        r#"[kitteh]
name = "{}"
version = "0.1.0"
description = "a very important kitteh project"
author = "Anonymous Kitteh"

[dependencies]
# add ur dependencies here!
# example:
# fancy_feast = "1.0"

[treats]
# extra compiler flags
# optimization = "max_zoomies"
"#,
        project_name
    );

    fs::write(project_dir.join(KIBBLE_FILE), kibble_toml)
        .map_err(|e| format!("couldnt write Kibble.toml: {}", e))?;

    let main_meow = r#"iz main() {
    meow!("Oh hai! Welcome to mai project! :3");
}
"#;

    fs::write(project_dir.join("src").join("main.meow"), main_meow)
        .map_err(|e| format!("couldnt write main.meow: {}", e))?;

    let gitignore = format!("{}/\n*.exe\n*.rs\n", LITTER_BOX);
    fs::write(project_dir.join(".gitignore"), gitignore)
        .map_err(|e| format!("couldnt write .gitignore: {}", e))?;

    println!("   /\\_/\\");
    println!("  ( ^.^ )  Created new project '{}'!", project_name);
    println!("   > ^ <");
    println!("  /|   |\\  Files created:");
    println!(" (_|   |_)");
    println!();
    println!("  {}/", project_name);
    println!("  ├── Kibble.toml");
    println!("  ├── .gitignore");
    println!("  └── src/");
    println!("      └── main.meow");
    println!();
    println!("  Run `lolrust kibble build` to compile!");

    Ok(())
}

/// Build the current project
pub fn build(explain: bool) -> Result<(), String> {
    let config = read_kibble_toml()?;
    let name = config.get("name")
        .ok_or("Kibble.toml iz missing 'name' in [kitteh]!")?;

    println!("   /\\_/\\");
    println!("  ( o.o )  Building '{}'...", name);
    println!("   > ^ <");
    println!();

    // Find all .meow files in src/
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return Err("no src/ directory found! where iz ur code kitteh?".to_string());
    }

    let meow_files = find_meow_files(src_dir)?;
    if meow_files.is_empty() {
        return Err("no .meow files found in src/! did u forget to write code?".to_string());
    }

    // Create litter_box directory
    let litter_box = Path::new(LITTER_BOX);
    fs::create_dir_all(litter_box)
        .map_err(|e| format!("couldnt make litter_box: {}", e))?;

    // Transpile all files
    println!("  Transpiling {} .meow file(s)...", meow_files.len());
    let mut rs_files = Vec::new();
    for meow_file in &meow_files {
        let source = fs::read_to_string(meow_file)
            .map_err(|e| format!("couldnt read {}: {}", meow_file.display(), e))?;

        let rust_code = crate::transpiler::transpile(&source);

        let rs_name = meow_file.with_extension("rs");
        let rs_name = rs_name.file_name().unwrap();
        let rs_path = litter_box.join(rs_name);

        fs::write(&rs_path, &rust_code)
            .map_err(|e| format!("couldnt write {}: {}", rs_path.display(), e))?;

        println!("    {} -> {}", meow_file.display(), rs_path.display());
        rs_files.push(rs_path);
    }

    // Find the main file (main.rs in litter_box)
    let main_rs = litter_box.join("main.rs");
    if !main_rs.exists() {
        return Err("no src/main.meow found! i need a main.meow to build kitteh".to_string());
    }

    // Compile with rustc
    let output_name = if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };
    let output_path = litter_box.join(&output_name);

    println!("  Compiling...");
    let result = Command::new("rustc")
        .arg(&main_rs)
        .arg("-o")
        .arg(&output_path)
        .output()
        .map_err(|e| format!("couldnt run rustc: {}. iz it installed?", e))?;

    if result.status.success() {
        println!();
        println!("  Build succeeded! :3");
        println!("  Binary: {}", output_path.display());

        // Write lock file
        write_kibble_lock(name, &meow_files)?;
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        if explain {
            eprintln!("{}", crate::explain::explain_errors(&stderr));
        } else {
            eprintln!("{}", stderr);
            eprintln!();
            eprintln!("  Build failed! D:");
            eprintln!("  (tip: use `lolrust kibble build --explain` for lolcat errors)");
        }
        return Err("build failed".to_string());
    }

    Ok(())
}

/// Run the built project
pub fn run(explain: bool) -> Result<(), String> {
    let config = read_kibble_toml()?;
    let name = config.get("name")
        .ok_or("Kibble.toml iz missing 'name' in [kitteh]!")?;

    let output_name = if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };
    let binary = Path::new(LITTER_BOX).join(&output_name);

    if !binary.exists() {
        println!("  No binary found, building first...");
        println!();
        build(explain)?;
    }

    println!();
    println!("   /\\_/\\");
    println!("  ( ^o^ )  Running '{}'...", name);
    println!("   > ^ <");
    println!("  ~~~~~~~~~~~~~~~~~~~~~~~~");
    println!();

    let status = Command::new(&binary)
        .status()
        .map_err(|e| format!("couldnt run {}: {}", binary.display(), e))?;

    if !status.success() {
        return Err(format!("program exited with code {}", status.code().unwrap_or(-1)));
    }

    Ok(())
}

/// Clean the litter box (remove build artifacts)
pub fn clean() -> Result<(), String> {
    let litter_box = Path::new(LITTER_BOX);
    if litter_box.exists() {
        fs::remove_dir_all(litter_box)
            .map_err(|e| format!("couldnt clean litter_box: {}", e))?;
    }

    if Path::new(KIBBLE_LOCK).exists() {
        fs::remove_file(KIBBLE_LOCK)
            .map_err(|e| format!("couldnt remove Kibble.lock: {}", e))?;
    }

    println!("   /\\_/\\");
    println!("  ( -.- )  Litter box cleaned! All sparkly now.");
    println!("   > ^ <");

    Ok(())
}

/// Show project info
pub fn info() -> Result<(), String> {
    let config = read_kibble_toml()?;

    println!("   /\\_/\\");
    println!("  ( o.o )  Project Info");
    println!("   > ^ <");
    println!();

    for (key, value) in &config {
        println!("  {}: {}", key, value);
    }

    let src_dir = Path::new("src");
    if src_dir.exists() {
        let meow_files = find_meow_files(src_dir).unwrap_or_default();
        println!();
        println!("  .meow files: {}", meow_files.len());
        for f in &meow_files {
            println!("    - {}", f.display());
        }
    }

    let litter_box = Path::new(LITTER_BOX);
    if litter_box.exists() {
        println!();
        println!("  litter_box: exists (built)");
    } else {
        println!();
        println!("  litter_box: empty (not built yet)");
    }

    Ok(())
}

fn read_kibble_toml() -> Result<HashMap<String, String>, String> {
    let content = fs::read_to_string(KIBBLE_FILE)
        .map_err(|_| "no Kibble.toml found! iz u in a kibble project? run `lolrust kibble init` first".to_string())?;

    let mut config = HashMap::new();
    let mut in_kitteh_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[kitteh]" {
            in_kitteh_section = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_kitteh_section = false;
            continue;
        }
        if in_kitteh_section && trimmed.contains('=') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().trim_matches('"').to_string();
                config.insert(key, value);
            }
        }
    }

    Ok(config)
}

fn find_meow_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("couldnt read {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("error reading directory: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "meow" {
                    files.push(path);
                }
            }
        } else if path.is_dir() {
            files.extend(find_meow_files(&path)?);
        }
    }

    Ok(files)
}

fn write_kibble_lock(name: &str, files: &[PathBuf]) -> Result<(), String> {
    let mut lock_content = String::new();
    lock_content.push_str("# DO NOT EDIT - generated by kibble\n");
    lock_content.push_str(&format!("# project: {}\n", name));
    lock_content.push_str(&format!("# built: {}\n\n", chrono_lite_now()));
    lock_content.push_str("[files]\n");
    for f in files {
        lock_content.push_str(&format!("{}\n", f.display()));
    }
    fs::write(KIBBLE_LOCK, lock_content)
        .map_err(|e| format!("couldnt write Kibble.lock: {}", e))?;
    Ok(())
}

fn chrono_lite_now() -> String {
    // Simple timestamp without pulling in chrono crate
    use std::time::SystemTime;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => format!("unix:{}", d.as_secs()),
        Err(_) => "unknown".to_string(),
    }
}
