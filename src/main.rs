use anyhow::Result;
use clap::Parser;
use gimp::gimp;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> {
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
