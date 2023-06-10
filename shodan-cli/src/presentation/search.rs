use super::command::Command;
use anyhow::Error;
use async_trait::async_trait;
use shodan_api::Datasource;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[async_trait]
impl Command for Opt {
    async fn run(&self, http_client: &reqwest::Client, api_key: &str) -> Result<(), Error> {
        let datasource = shodan_api::ShodanApi::new(http_client, api_key);

        let response = match &self.subcommand {
            Subcommand::Ip(opt) => datasource.search_ip(&opt.ip).await?,
        };

        println!("{:#?}", response);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Ip(Ip),
}

#[derive(Debug, StructOpt)]
pub struct Ip {
    pub ip: String,
}
