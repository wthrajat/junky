use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[command(
    name = "junky",
    version,
    about = "Junky, a safe scan-first junk cleaner for Windows"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub include_aggressive: bool,

    #[arg(long, global = true, value_name = "HOURS")]
    pub older_than_hours: Option<u64>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Scan(ScanArgs),
    Clean(CleanArgs),
    List,
}

#[derive(Debug, clap::Args)]
pub struct ScanArgs {
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
}

#[derive(Debug, clap::Args)]
pub struct CleanArgs {
    #[arg(long)]
    pub yes: bool,
}
