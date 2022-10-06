use clap::Parser;
use reqwest;

mod cli;
mod filters;
mod types;

use crate::{cli::CliOpts, filters::Filters, types::MirrorsStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliOpts::parse();
    dbg!("{:?}", &args);

    let response = reqwest::blocking::get(&args.url)?;
    let data: MirrorsStatus = response.json()?;
    dbg!("{:#?}", &data.urls);

    let fmirrors = Filters::new(data.urls).apply_filters(&args.filter_opts);
    dbg!("{:#?}", &fmirrors);

    let mirrors_url = fmirrors.into_iter().map(|x| x.url).collect::<Vec<String>>();

    Ok(())
}
