#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorCode {
    Success = 0,
    Failure = 1,

    #[default]
    Unknown = 2,

    // Peer Errors
    PeerDropped = 16,
}
