pub struct WARequest {
    pub council_id: u8,
    pub resolution_id: u32,
    pub shards: Vec<WAGeneralShard>,
}

pub enum WAGeneralShard {
    NumNations,
    NumDelegates,
    Delegates,
    Members,
    Happenings,
    Proposals,
    Resolution {
        voters: bool,
        vote_track: bool,
        delegate_log: bool,
        delegate_votes: bool,
        last_resolution: bool,
    },
}
