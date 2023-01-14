pub enum Error {
    Corrupted,
    ByteReadError,
    ChunckSplitError,
}

impl std::error::Error for Error {}
