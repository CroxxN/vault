pub enum Error {
    Corrupted,
    ByteReadError,
    ChunckSplitError,
}

impl Error for Error {}
