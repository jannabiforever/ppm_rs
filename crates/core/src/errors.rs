#[derive(Debug, Clone, PartialEq, thiserror::Error)]

pub enum PPMError {}

pub(crate) type PPMResult<T> = Result<T, PPMError>;
