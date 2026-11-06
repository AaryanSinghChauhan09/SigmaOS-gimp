use ::gimp::{gimp, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = gimp::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-gimp operational");
    Ok(())
}
