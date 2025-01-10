use clap::Parser;

#[derive(Parser)]
#[command(version = "0.1")]
#[command(about = "\x1B[32mA weather forecast CLI tool\x1B[0m", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub place: Option<String>,
    #[arg(long)]
    pub latitude: Option<f64>,
    #[arg(long)]
    pub longitude: Option<f64>,
    
}