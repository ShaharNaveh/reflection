use crate::{
    cli::{FilterOpts, ProtocolOpts},
    types::MirrorMetadata,
};

use chrono;

pub struct Filters(Vec<MirrorMetadata>);

impl Filters {
    pub fn new(v: Vec<MirrorMetadata>) -> Self {
        Self(v)
    }

    pub fn apply_filters(mut self, filter_opts: &FilterOpts) -> Vec<MirrorMetadata> {
        self.0.retain(|x| x.active); // Get only active mirrors, should we always do this?

        if let Some(age) = filter_opts.age {
            self.filter_age(age);
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

    fn filter_completion_percent(&mut self, completion_pct: u8) {
        let completion_pct: f32 = completion_pct as f32 / 100.0;
        self.0.retain(|x| x.completion_pct >= completion_pct);
    }

    fn filter_protocols(&mut self, protocols: Vec<ProtocolOpts>) {
        let wanted_protocols = protocols
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        self.0
            .retain(|x| wanted_protocols.contains(&x.protocol.to_string()));
    }
}
