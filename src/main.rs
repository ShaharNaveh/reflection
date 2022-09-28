use clap::Parser;
use reqwest;

mod cli;
mod types;

use crate::{
    cli::{Opt, ProtocolOpts},
    types::{MirrorMetadata, MirrorStatus},
};

const API_URL: &str = "https://archlinux.org/mirrors/status/json/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::parse();

    let filter_opts = &args.filter_opts;

    let completion_pct: f32 = filter_opts.completion_percent as f32 / 100.0;

    let wanted_protocols: Vec<ProtocolOpts> = if filter_opts.protocol.is_none() {
        vec![
            ProtocolOpts::Ftp,
            ProtocolOpts::Http,
            ProtocolOpts::Https,
            ProtocolOpts::Rsync,
        ]
    } else {
        filter_opts.protocol.as_ref().unwrap().to_vec()
    };

    let protocols: Vec<&str> = wanted_protocols.into_iter().map(|x| x.to_str()).collect();

    let response = reqwest::blocking::get(API_URL)?;
    let data: MirrorStatus = response.json()?;

    let mirrors_status = data.urls;

    let fmirrors = mirrors_status
        .into_iter()
        .filter(|x| x.active) // Get only active mirrors, should always?
        .filter(|x| x.completion_pct >= completion_pct)
        .filter(|x| protocols.contains(&x.protocol.as_str()))
        .collect::<Vec<MirrorMetadata>>();

    let mirrors_url: Vec<String> = fmirrors.into_iter().map(|x| x.url).collect();

    Ok(())
}
