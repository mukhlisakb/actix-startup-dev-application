use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database connection failed: {0}")]
    DatabaseConnectionError(String),

    #[error("Database query failed: {0}")]
    DatabaseQueryError(String),

    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
