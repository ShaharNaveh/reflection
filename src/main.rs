use clap::Parser;

mod cli;
mod consts;
mod types;
mod utils;

use crate::cli::CliOpts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = dbg!(CliOpts::parse());

    let mirrorstatus =
        utils::get_mirrorstatus(args.connection_timeout, args.cache_timeout, &args.url);

    let fmirrors = mirrorstatus.filter_by_opts(&args.filter_opts);
    //println!("{:#?}", &fmirrors);

    let mirrors = fmirrors.urls();
    //println!("{:#?}", &mirrors);

    Ok(())
}
