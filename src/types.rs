use clap::ValueEnum;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MirrorMetadata {
    pub url: String,
    pub protocol: Protocol,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub completion_pct: f32,
    pub delay: Option<usize>,
    pub duration_avg: Option<f32>,
    pub duration_stddev: Option<f32>,
    pub score: Option<f32>,
    pub active: bool,
    pub country: String,
    pub country_code: String,
    pub isos: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MirrorsStatus {
    pub cutoff: usize,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub num_checks: usize,
    pub check_frequency: usize,
    pub urls: Vec<MirrorMetadata>,
    pub version: usize,
}

impl MirrorsStatus {
    pub fn urls(self) -> Vec<String> {
        self.urls.into_iter().map(|x| x.url).collect()
    }

    pub fn filter_by_opts(mut self, filter_opts: &crate::cli::FilterOpts) -> Self {
        // Should we always filter out inactive mirrors?
        self.filter_active();

        if filter_opts.ipv4 {
            self.filter_ipv4();
        }

        if filter_opts.ipv6 {
            self.filter_ipv6();
        }

        if filter_opts.isos {
            self.filter_isos();
        }

        self.filter_completion_percent(filter_opts.completion_percent);

        if let Some(protocols) = &filter_opts.protocol {
            self.filter_protocols(protocols.to_vec());
        }

        if let Some(age) = filter_opts.age {
            self.filter_age(age);
        }

        if let Some(re) = &filter_opts.include {
            self.filter_include_re(&re);
        }

        if let Some(re) = &filter_opts.exclude {
            self.filter_exclude_re(&re);
        }

        if let Some(latest) = filter_opts.latest {
            self.n_latest(latest);
        }

        if let Some(number) = filter_opts.number {
            self = self.n_urls(number);
        }

        self
    }

    fn n_urls(mut self, number: usize) -> Self {
        self.urls = self.urls.into_iter().take(number).collect();
        self
    }

    fn filter_active(&mut self) {
        self.urls.retain(|x| x.active)
    }

    fn n_latest(&mut self, latest: usize) {
        let urls = {
            let mut data = self.urls.clone();
            data.sort_unstable_by_key(|x| Reverse(x.last_sync));
            data
        };
        self.urls = urls[..latest].to_vec();
    }

    fn filter_ipv4(&mut self) {
        self.urls.retain(|x| x.ipv4)
    }

    fn filter_ipv6(&mut self) {
        self.urls.retain(|x| x.ipv6)
    }

    fn filter_isos(&mut self) {
        self.urls.retain(|x| x.isos)
    }

    fn filter_include_re(&mut self, re: &Regex) {
        self.urls.retain(|x| re.is_match(&x.url))
    }

    fn filter_exclude_re(&mut self, re: &Regex) {
        self.urls.retain(|x| !re.is_match(&x.url))
    }

    fn filter_completion_percent(&mut self, completion_pct: u8) {
        let completion_pct: f32 = completion_pct as f32 / 100.0;
        self.urls.retain(|x| x.completion_pct >= completion_pct)
    }

    fn filter_age(&mut self, age: f64) {
        let utc = chrono::Utc::now();
        self.urls.retain(|x| match &x.last_sync {
            Some(v) => (age * 60.0 * 60.0) as i64 >= utc.signed_duration_since(*v).num_seconds(),
            None => false,
        })
    }

    fn filter_protocols(&mut self, protocols: Vec<Protocol>) {
        self.urls.retain(|x| protocols.contains(&x.protocol))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Ftp,
    Http,
    Https,
    Rsync,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let protocol = format!("{:?}", self).to_lowercase();
        write!(f, "{}", protocol)
    }
}
