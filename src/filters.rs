use crate::types::MirrorMetadata;
use regex::Regex;

pub struct Filters(Vec<MirrorMetadata>);

impl Filters {
    pub fn new(v: Vec<MirrorMetadata>) -> Self {
        Self(v)
    }

    pub fn apply_filters(mut self, filter_opts: &crate::cli::FilterOpts) -> Vec<MirrorMetadata> {
        self.0.retain(|x| x.active); // Get only active mirrors, should we always do this?

        if let Some(age) = filter_opts.age {
            self.filter_age(age);
        }

        if let Some(include_re) = &filter_opts.include {
            self.filter_include_re(&include_re);
        }

        if let Some(exclude_re) = &filter_opts.exclude {
            self.filter_exclude_re(&exclude_re);
        }

        if let Some(protocol) = &filter_opts.protocol {
            self.filter_protocols(protocol.to_vec());
        }

        self.filter_completion_percent(filter_opts.completion_percent);

        self.0
    }

    fn filter_age(&mut self, age: f64) {
        let utc = chrono::Utc::now();

        self.0.retain(|x| match &x.last_sync {
            Some(v) => (age * 60.0 * 60.0) as i64 >= utc.signed_duration_since(*v).num_seconds(),
            None => false,
        })
    }

    fn filter_include_re(&mut self, re: &Regex) {
        self.0.retain(|x| re.is_match(&x.url));
    }

    fn filter_exclude_re(&mut self, re: &Regex) {
        self.0.retain(|x| !re.is_match(&x.url));
    }

    fn filter_completion_percent(&mut self, completion_pct: u8) {
        let completion_pct: f32 = completion_pct as f32 / 100.0;
        self.0.retain(|x| x.completion_pct >= completion_pct);
    }

    fn filter_protocols(&mut self, protocols: Vec<crate::types::Protocol>) {
        self.0.retain(|x| protocols.contains(&x.protocol));
    }
}
