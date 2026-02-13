/// Errors produced by the em-core library.
#[derive(Debug, thiserror::Error)]
pub enum EmCoreError {
    #[error("invalid parameter '{name}': {reason}")]
    InvalidParameter { name: String, reason: String },

    #[error("coordinate transform failed: {0}")]
    CoordinateTransform(String),

    #[error("division by zero in {context}")]
    DivisionByZero { context: String },

    #[error("value out of range: {name} = {value}, expected {expected}")]
    OutOfRange {
        name: String,
        value: f64,
        expected: String,
    },

    #[error("numerical convergence failed after {iterations} iterations")]
    ConvergenceFailed { iterations: usize },
}

/// Convenience result type for em-core operations.
pub type EmCoreResult<T> = Result<T, EmCoreError>;
