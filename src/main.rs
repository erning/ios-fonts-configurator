use clap::Parser;
use mobileconfig::MobileConfig;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

mod mobileconfig;

/// Supported font file extensions
const FONT_EXTENSIONS: &[&str] = &["ttf", "otf", "woff", "woff2"];

/// Generate .mobileconfig files for iOS font installation
#[derive(Parser)]
#[command(name = "ifonts", version, about)]
struct Args {
    /// Output .mobileconfig file path
    #[arg(short, long, value_name = "FILE")]
    output: String,

    /// Display name for the font profile
    #[arg(short, long, value_name = "NAME")]
    name: String,

    /// Unique identifier (e.g., com.example.fonts)
    #[arg(short, long, value_name = "IDENTIFIER")]
    identifier: String,

    /// Font files or directories containing fonts (use "-" to read from stdin)
    #[arg(short, long, value_name = "PATHS", num_args = 1..)]
    fonts: Vec<String>,

    /// Maximum directory recursion depth
    #[arg(short = 'd', long, value_name = "DEPTH", default_value = "3")]
    max_depth: u32,

    /// Suppress output messages
    #[arg(short, long)]
    quiet: bool,
}

fn is_font_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| FONT_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

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

        if entry_path.is_file() && is_font_file(&entry_path) {
            font_files.push(entry_path.to_string_lossy().into_owned());
        } else if entry_path.is_dir() {
            collect_font_files_recursive(&entry_path, font_files, current_depth + 1, max_depth)?;
        }
    }

    Ok(())
}

fn collect_font_files(paths: &[String], max_depth: u32) -> anyhow::Result<Vec<String>> {
    let mut font_files = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
        }

        if path.is_dir() {
            collect_font_files_recursive(path, &mut font_files, 0, max_depth)?;
        } else if path.is_file() {
            if is_font_file(path) {
                font_files.push(path.to_string_lossy().into_owned());
            } else {
                return Err(anyhow::anyhow!(
                    "Not a supported font file ({}): {}",
                    FONT_EXTENSIONS.join(", "),
                    path.display()
                ));
            }
        }
    }

    if font_files.is_empty() {
        return Err(anyhow::anyhow!(
            "No font files found in the specified paths"
        ));
    }

    Ok(font_files)
}

fn read_paths_from_stdin() -> anyhow::Result<Vec<String>> {
    let stdin = io::stdin();
    let paths: Vec<String> = stdin
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if paths.is_empty() {
        return Err(anyhow::anyhow!("No paths provided from stdin"));
    }

    Ok(paths)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let font_paths = if args.fonts.iter().any(|p| p == "-") {
        if args.fonts.len() != 1 {
            return Err(anyhow::anyhow!(
                "'-' (stdin) cannot be mixed with other paths in --fonts"
            ));
        }
        read_paths_from_stdin()?
    } else {
        args.fonts
    };

    let output_path = Path::new(&args.output);

    if !args.quiet {
        println!("Creating mobileconfig file: {}", output_path.display());
        println!("Profile name: {}", args.name);
        println!("Identifier: {}", args.identifier);
        println!("Input paths: {:?}", font_paths);
    }

    let font_files = collect_font_files(&font_paths, args.max_depth)?;

    let mut config = MobileConfig::new(args.name, args.identifier);

    for font_file in font_files {
        let font_path = Path::new(&font_file);
        if let Err(e) = config.add_font(font_path, None) {
            return Err(anyhow::anyhow!(
                "Failed to add font {}: {}",
                font_path.display(),
                e
            ));
        }

        if !args.quiet {
            println!("Added font: {}", font_path.display());
        }
    }

    config.save_to_file(output_path)?;

    if !args.quiet {
        println!(
            "Successfully generated .mobileconfig file: {}",
            output_path.display()
        );
    }

    Ok(())
}
