//! Rusty JVM Main Entry Point

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use rusty_jvm::{init_logger, Result, JVM_VERSION};

#[derive(Parser)]
#[command(name = "rusty_jvm")]
#[command(about = "A JVM implementation in Rust", version = JVM_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Java class file
    Run {
        /// Class file path
        #[arg(value_name = "CLASS_FILE")]
        class_file: PathBuf,

        /// Classpath
        #[arg(short, long)]
        classpath: Option<String>,

        /// input params
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Disassemble class file
    Disassemble {
        /// Class file path
        #[arg(value_name = "CLASS_FILE")]
        class_file: PathBuf,

        /// display detail data
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    init_logger();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { class_file, classpath, args } => {
            log::info!("Running class file: {:?}", class_file);
            log::info!("Classpath: {:?}", classpath);
            log::info!("Arguments: {:?}", args);

            // TODO: 實現 JVM 運行邏輯
            println!("JVM is not implemented yet!");
            Ok(())
        }

        Commands::Disassemble { class_file, verbose } => {
            log::info!("Disassembling class file: {:?}", class_file);

            // TODO: 實現反組譯邏輯
            println!("Disassembler is not implemented yet!");
            Ok(())
        }
    }
}