use crate::{
    cli::{FilterOpts, ProtocolOpts},
    types::MirrorMetadata,
};

pub fn apply_filters(
    filter_opts: &FilterOpts,
    mirrors_status: &Vec<MirrorMetadata>,
) -> Vec<MirrorMetadata> {
    let mut filtered_mirrors: Vec<MirrorMetadata> = mirrors_status.to_vec().clone();

    filtered_mirrors = filtered_mirrors
        .into_iter()
        .filter(|x| x.active) // Get only active mirrors, should we always do this?
        .collect();

    filtered_mirrors = filter_completion_percent(&filter_opts.completion_percent, filtered_mirrors);
    filtered_mirrors = filter_protocols(filter_opts.protocol.as_ref(), filtered_mirrors);

    filtered_mirrors
}

fn filter_completion_percent(
    completion_pct: &u8,
    mirrors_status: Vec<MirrorMetadata>,
) -> Vec<MirrorMetadata> {
    let completion_pct: f32 = *completion_pct as f32 / 100.0;

    let fmirrors = mirrors_status
        .into_iter()
        .filter(|x| x.completion_pct >= completion_pct)
        .collect();

    fmirrors
}

fn filter_protocols(
    protocols: Option<&Vec<ProtocolOpts>>,
    mirrors_status: Vec<MirrorMetadata>,
) -> Vec<MirrorMetadata> {
    if protocols.is_none() {
        return mirrors_status;
    }
    let protocols = protocols.unwrap();
    let wanted_protocols = protocols
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let fmirrors = mirrors_status
        .into_iter()
        .filter(|x| wanted_protocols.contains(&x.protocol.to_string()))
        .collect();

    fmirrors
}
