use std::error::Error;
use std::fmt;
pub mod bls12381;
pub mod zexe;

#[derive(Debug)]
pub struct InvalidLength(usize, usize);

impl fmt::Display for InvalidLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decoding: invalid length {}/{}", self.0, self.1)
    }
}

impl Error for InvalidLength {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        // TODO
        None
    }
}
