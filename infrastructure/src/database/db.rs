use log::{error, info};
use shared::config::DatabaseConfig;
use shared::error::AppError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database connection manager
#[derive(Clone, Debug)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Creates a new database connection pool
    ///
    /// # Arguments
    ///
    /// * `config` - Database configuration
    ///
    /// # Returns
    ///
    /// * `Result<Self, AppError>` - Database instance or error
    pub async fn new_connection(config: &DatabaseConfig) -> Result<Self, AppError> {
        info!("Creating database connection pool...");

        let pool = PgPoolOptions::new()
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_millis(config.connect_timeout_ms))
            .idle_timeout(Duration::from_millis(config.idle_timeout_ms))
            .connect(&config.connection_string())
            .await
            .map_err(|e| {
                error!("Failed to create database pool: {}", e);
                AppError::DatabaseConnectionError(e.to_string())
            })?;

        info!("Database connection pool created successfully");
        Ok(Self { pool })
    }

    /// Get a connection from the pool
    ///
    /// Note: sqlx handles connection pooling automatically.
    /// This method is a wrapper to demonstrate the requirement.
    /// When you use `&self.pool` in queries, it automatically acquires and releases connections.
    pub async fn get_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, AppError> {
        match self.pool.acquire().await {
            Ok(conn) => {
                info!("Connection acquired from pool");
                Ok(conn)
            }
            Err(e) => {
                error!("Failed to acquire connection from pool: {}", e);
                Err(AppError::DatabaseConnectionError(e.to_string()))
            }
        }
    }

    /// Release a connection back to the pool
    ///
    /// Note: In sqlx/Rust, connections are automatically released when they go out of scope (RAII).
    /// This function is provided to satisfy the requirement but is not typically needed in idiomatic Rust.
    pub fn release_connection(_conn: sqlx::pool::PoolConnection<sqlx::Postgres>) {
        // Connection is dropped here and released to the pool
        info!("Connection released back to pool (via RAII)");
    }

    /// Shutdown the connection pool
    pub async fn shutdown(&self) {
        info!("Shutting down database connection pool...");
        self.pool.close().await;
        info!("Database connection pool closed");
    }
}
