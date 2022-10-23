use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum};

use regex::Regex;

use crate::types::Protocol;

/// retrieve and filter a list of the latest Arch Linux mirrors
#[derive(Parser, Debug)]
#[command(name = "rflector")] // TODO: Find a better name
#[command(version = "0.0.1")]
#[command(next_line_help = true)]
pub struct CliOpts {
    /// The number of seconds to wait before a connection times out
    #[arg(long, value_name = "n", default_value_t = 5)]
    pub connection_timeout: usize,

    /// The number of seconds to wait before a download times out
    #[arg(long, value_name = "n", default_value_t = 5)]
    pub download_timeout: usize,

    /// Display a table of the distribution of servers by country
    #[arg(long, hide = true)]
    pub list_countries: bool,

    /// The cache timeout in seconds for the data retrieved from the Arch Linux Mirror Status API
    #[arg(long, value_name = "n", default_value_t = 300)]
    pub cache_timeout: usize,

    /// The URL from which to retrieve the mirror data in JSON format. If different from the default, it must follow the same format
    #[arg(
        long,
        value_name = "URL",
        default_value = crate::consts::URL
    )]
    pub url: String,

    /// Save the mirrorlist to the given path
    #[arg(long, value_name = "filepath", hide = true)]
    pub save: Option<PathBuf>,

    /// Sort the mirrorlist. "age": last server synchronization; "rate": download rate; "country": country name, either alphabetically or in the order given by the --country option; "score": MirrorStatus score; "delay": MirrorStatus delay
    #[arg(long, value_enum, ignore_case = true, hide = true)]
    pub sort: Option<SortOpts>,

    /// Use n threads for rating mirrors. This option will speed up the rating step but the results will be inaccurate if the local bandwidth is saturated at any point during the operation. If rating takes too long without this option then you should probably apply more filters to reduce the number of rated servers before using this option
    #[arg(long, value_name = "n", hide = true)]
    pub threads: Option<usize>,

    /// Print extra information to STDERR. Only works with some options
    #[arg(long, hide = true)]
    pub verbose: bool,

    /// Print mirror information instead of a mirror list. Filter options apply
    #[arg(long, hide = true)]
    pub info: bool,

    #[command(flatten, next_help_heading = "FILTERS")]
    pub filter_opts: FilterOpts,
}

#[derive(Args, Debug)]
pub struct FilterOpts {
    /// Only return mirrors that have synchronized in the last n hours. n may be an integer or a decimal number
    #[arg(short, long, value_name = "n")]
    pub age: Option<f64>,

    /// Only return mirrors with a reported sync delay of n hours or less, where n is a float. For example. to limit the results to mirrors with a reported delay of 15 minute or less, pass 0.25.
    #[arg(short, long, value_name = "n", hide = true)]
    pub delay: Option<f64>,

    /// Include servers that match <regex>, where <regex> is a *Rust* regular expression.
    #[arg(short, long, value_name = "regex", value_parser=Regex::new)]
    pub include: Option<Regex>,

    /// Include servers that match <regex>, where <regex> is a *Rust* regular expression.
    #[arg(short = 'x', long, value_name = "regex", value_parser=Regex::new)]
    pub exclude: Option<Regex>,

    /// Limit the list to the n most recently synchronized servers
    #[arg(short, long, value_name = "n")]
    pub latest: Option<usize>,

    /// Return at most n mirrors
    #[arg(short, long, value_name = "n")]
    pub number: Option<usize>,

    /// Match one of the given protocols, e.g. "https" or "ftp". Multiple protocols may be selected using commas (e.g. "https,http") or by passing this option multiple times.
    #[arg(
        short,
        long,
        value_name = "protocol",
        value_enum,
        value_delimiter = ','
    )]
    pub protocol: Option<Vec<Protocol>>,

    /// Set the minimum completion percent for the returned mirrors. Check the mirrorstatus webpage for the meaning of this parameter
    #[arg(
        long,
        value_name = "n",
        value_parser = clap::value_parser!(u8).range(0..101),
        default_value_t = 100
    )]
    pub completion_percent: u8,

    /// Only return mirrors that host ISOs
    #[arg(long)]
    pub isos: bool,

    /// Only return mirrors that support IPv4
    #[arg(long)]
    pub ipv4: bool,

    /// Only return mirrors that support IPv6
    #[arg(long)]
    pub ipv6: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum SortOpts {
    Age,
    Rate,
    Country,
    Score,
    Delay,
}
