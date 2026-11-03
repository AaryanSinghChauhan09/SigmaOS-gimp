use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use gimp::gimp;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    let config = gimp::Config { 
        desktop_integration: !cli.no_integration, 
        ..Default::default() 
    };
    let mut component = gimp::new(config);
    
    component.initialize()?;
    component.launch()?;
    
    println!("SigmaOS-gimp ready");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
