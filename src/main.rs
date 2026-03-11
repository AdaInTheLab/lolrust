use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

use lolrust::explain::explain_errors;
use lolrust::kibble;
use lolrust::logo::LOGO;
use lolrust::transpiler::transpile;

#[derive(Parser)]
#[command(name = "lolrust", version, about = LOGO)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input .meow file to transpile (when not using subcommands)
    #[arg(global = false)]
    input: Option<PathBuf>,

    /// Output transpiled .rs file instead of compiling
    #[arg(long)]
    emit_rs: bool,

    /// Compile and immediately run the result
    #[arg(long)]
    run: bool,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Show transpiled Rust code before compiling
    #[arg(short, long)]
    verbose: bool,

    /// Translate rustc errors into lolcat speak
    #[arg(long)]
    explain: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// kibble - the lolrust package manager :3
    Kibble {
        #[command(subcommand)]
        action: KibbleAction,
    },
}

#[derive(Subcommand)]
enum KibbleAction {
    /// Create a new kitteh project
    Init {
        /// Project name
        name: Option<String>,
    },
    /// Build the current project
    Build {
        /// Translate errors to lolcat speak
        #[arg(long)]
        explain: bool,
    },
    /// Build and run the current project
    Run {
        /// Translate errors to lolcat speak
        #[arg(long)]
        explain: bool,
    },
    /// Clean the litter box (remove build artifacts)
    Clean,
    /// Show project info
    Info,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Kibble { action }) => handle_kibble(action),
        None => handle_transpile(cli),
    }
}

fn handle_kibble(action: KibbleAction) {
    let result = match action {
        KibbleAction::Init { name } => kibble::init(name.as_deref()),
        KibbleAction::Build { explain } => kibble::build(explain),
        KibbleAction::Run { explain } => kibble::run(explain),
        KibbleAction::Clean => kibble::clean(),
        KibbleAction::Info => kibble::info(),
    };

    if let Err(e) = result {
        eprintln!();
        eprintln!("   /\\_/\\");
        eprintln!("  ( x.x )  {}", e);
        eprintln!("   > ^ <");
        process::exit(1);
    }
}

fn handle_transpile(args: Cli) {
    let input = match args.input {
        Some(p) => p,
        None => {
            eprintln!("Error: no input file! usage: lolrust <file.meow>");
            eprintln!("  or try: lolrust kibble --help");
            process::exit(1);
        }
    };

    // Validate .meow extension
    match input.extension().and_then(|e| e.to_str()) {
        Some("meow") => {}
        _ => {
            eprintln!("Error: input file must have .meow extension, ya silly kitteh!");
            process::exit(1);
        }
    }

    // Read source
    let source = match fs::read_to_string(&input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {:?}: {}", input, e);
            process::exit(1);
        }
    };

    // Transpile
    let rust_code = transpile(&source);

    if args.verbose {
        eprintln!("=== Transpiled Rust code ===");
        eprintln!("{}", rust_code);
        eprintln!("============================");
    }

    if args.emit_rs {
        // Write .rs file
        let output_path = args
            .output
            .unwrap_or_else(|| input.with_extension("rs"));
        if let Err(e) = fs::write(&output_path, &rust_code) {
            eprintln!("Error writing {:?}: {}", output_path, e);
            process::exit(1);
        }
        println!("Wrote {}", output_path.display());
    } else {
        // Write temp file, compile with rustc
        let temp_dir = std::env::temp_dir();
        let temp_rs = temp_dir.join(format!("lolrust_{}.rs", process::id()));

        if let Err(e) = fs::write(&temp_rs, &rust_code) {
            eprintln!("Error writing temp file: {}", e);
            process::exit(1);
        }

        let output_bin = args.output.unwrap_or_else(|| {
            let mut p = input.with_extension("");
            if cfg!(windows) {
                p.set_extension("exe");
            }
            p
        });

        let result = Command::new("rustc")
            .arg(&temp_rs)
            .arg("-o")
            .arg(&output_bin)
            .output();

        // Clean up temp file
        let _ = fs::remove_file(&temp_rs);

        match result {
            Ok(output) if output.status.success() => {
                println!(
                    "Compiled {} -> {}  :3",
                    input.display(),
                    output_bin.display()
                );

                if args.run {
                    let run_status = Command::new(&output_bin).status();
                    match run_status {
                        Ok(s) => process::exit(s.code().unwrap_or(1)),
                        Err(e) => {
                            eprintln!("Error running {:?}: {}", output_bin, e);
                            process::exit(1);
                        }
                    }
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if args.explain {
                    eprintln!("{}", explain_errors(&stderr));
                } else {
                    eprintln!("{}", stderr);
                    eprintln!("rustc failed! oh noes! D:");
                    eprintln!("(tip: use --explain for lolcat error messages)");
                }
                process::exit(output.status.code().unwrap_or(1));
            }
            Err(e) => {
                eprintln!("Could not run rustc: {}. iz rustc installed?", e);
                process::exit(1);
            }
        }
    }
}
