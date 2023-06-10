use anyhow::Error;
use std::{env, time};
use structopt::StructOpt;

mod presentation;
use presentation::command::Command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, env = "SHODAN_API_KEY", hide_env_values = true)]
    api_key: String,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    Search(presentation::search::Opt),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Program started!");

    let opt = Opt::from_args();
    let http_client = reqwest::ClientBuilder::new()
        .timeout(time::Duration::from_secs(5))
        .build()?;

    let api_key = opt.api_key;

    match opt.subcommand {
        Subcommand::Search(opt) => opt.run(&http_client, &api_key).await?,
    };

    Ok(())
}
