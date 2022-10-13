use clap::Parser;
use reqwest;

mod cli;
mod types;

use crate::{cli::CliOpts, types::MirrorsStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliOpts::parse();
    dbg!("{:?}", &args);

    let response = reqwest::blocking::get(&args.url)?;
    let mirrors_status: MirrorsStatus = response.json()?;

    let fmirrors = mirrors_status.filter_by_opts(&args.filter_opts);
    println!("{:#?}", &fmirrors);

    let mirrors = fmirrors.urls();
    println!("{:#?}", &mirrors);

    /*
    let mirrors_url = fmirrors.into_iter().map(|x| x.url).collect::<Vec<String>>();
    println!("{:#?}", &mirrors_url);
    */

    Ok(())
}
