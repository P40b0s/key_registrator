
#[derive(Debug, thiserror::Error)]
pub enum Error 
{
    #[error("Unknown key => `{:#02x}`", 0)]
    UnknownKey(u32),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    //UtilitesError(#[from] utilites::error::Error),
}