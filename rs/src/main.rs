use clap::Parser;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = artisan_tools::cli::Cli::parse();
    log::debug!("{cli}");
    artisan_tools::run::command(cli.command)
}
