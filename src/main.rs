use clap::{Arg, Command};
use mobileconfig::MobileConfig;
use std::fs;
use std::path::Path;

mod mobileconfig;

fn collect_font_files_recursive(
    path: &Path,
    font_files: &mut Vec<String>,
    current_depth: u32,
    max_depth: u32,
) -> anyhow::Result<()> {
    if current_depth > max_depth {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(ext) = entry_path.extension().and_then(|s| s.to_str()) {
                if matches!(
                    ext.to_lowercase().as_str(),
                    "ttf" | "otf" | "woff" | "woff2"
                ) {
                    font_files.push(entry_path.to_string_lossy().into_owned());
                }
            }
        } else if entry_path.is_dir() {
            collect_font_files_recursive(&entry_path, font_files, current_depth + 1, max_depth)?;
        }
    }

    Ok(())
}

fn collect_font_files(paths: &[&String], max_depth: u32) -> anyhow::Result<Vec<String>> {
    let mut font_files = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
        }

        if path.is_dir() {
            collect_font_files_recursive(path, &mut font_files, 0, max_depth)?;
        } else if path.is_file() {
            font_files.push(path.to_string_lossy().into_owned());
        }
    }

    if font_files.is_empty() {
        return Err(anyhow::anyhow!(
            "No font files found in the specified paths"
        ));
    }

    Ok(font_files)
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new("ios-fonts-configurator")
        .version("0.1.0")
        .about("Generate .mobileconfig files for iOS font installation")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output .mobileconfig file path")
                .required(true),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Display name for the font profile")
                .required(true),
        )
        .arg(
            Arg::new("identifier")
                .short('i')
                .long("identifier")
                .value_name("IDENTIFIER")
                .help("Unique identifier (e.g., com.example.fonts)")
                .required(true),
        )
        .arg(
            Arg::new("fonts")
                .short('f')
                .long("fonts")
                .value_name("PATHS")
                .help("Font files or directories containing fonts (space-separated list)")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("max-depth")
                .short('d')
                .long("max-depth")
                .value_name("DEPTH")
                .help("Maximum directory recursion depth (default: 3)")
                .default_value("3"),
        )
        .get_matches();

    let output_path = Path::new(matches.get_one::<String>("output").unwrap());
    let display_name = matches.get_one::<String>("name").unwrap().clone();
    let identifier = matches.get_one::<String>("identifier").unwrap().clone();
    let font_paths: Vec<&String> = matches.get_many("fonts").unwrap().collect();
    let max_depth = matches
        .get_one::<String>("max-depth")
        .unwrap()
        .parse::<u32>()
        .map_err(|e| anyhow::anyhow!("Invalid max-depth value: {}", e))?;

    println!("Creating mobileconfig file: {}", output_path.display());
    println!("Profile name: {display_name}");
    println!("Identifier: {identifier}");
    println!("Input paths: {font_paths:?}");

    let font_files = collect_font_files(&font_paths, max_depth)?;
    // println!("Found font files: {:?}", font_files);

    let mut config = MobileConfig::new(display_name, identifier);

    for font_file in font_files {
        let font_path = Path::new(&font_file);
        if let Err(e) = config.add_font(font_path, None) {
            return Err(anyhow::anyhow!(
                "Failed to add font {}: {}",
                font_path.display(),
                e
            ));
        }

        println!("Added font: {}", font_path.display());
    }

    config.save_to_file(output_path)?;
    println!(
        "Successfully generated .mobileconfig file: {}",
        output_path.display()
    );

    Ok(())
}
