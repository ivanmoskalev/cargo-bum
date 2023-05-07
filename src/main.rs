use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ios(IosArgs),
    Android(AndroidArgs),
}

#[derive(Parser)]
struct IosArgs {
    input: String,
}

#[derive(Parser)]
struct AndroidArgs {
    input: String,
}

fn main() {
    let opts = Cli::parse();

    match opts.command {
        Commands::Ios(args) => build_ios(&args.input),
        Commands::Android(args) => build_android(&args.input),
    }
}

fn build_ios(input: &str) {
    println!("Building for iOS: {}", input);
    // Implement the iOS build logic here
}

fn build_android(input: &str) {
    println!("Building for Android: {}", input);
    // Implement the Android build logic here
}