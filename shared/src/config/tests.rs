#[cfg(test)]
mod tests {
    use crate::config::DatabaseConfig;

    #[test]
    fn test_default_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn test_connection_string() {
        let config = DatabaseConfig {
            host: "db_host".to_string(),
            port: 5432,
            username: "user".to_string(),
            password: Some("pass".to_string()),
            database_name: "dbname".to_string(),
            ..Default::default()
        };
        assert_eq!(
            config.connection_string(),
            "postgres://user:pass@db_host:5432/dbname"
        );
    }
}
