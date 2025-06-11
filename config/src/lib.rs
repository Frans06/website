use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid value for environment variable {key}: {value}")]
    InvalidEnvVar {
        key: String,
        value: String,
        reason: String,
    },

    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    #[error("Configuration validation failed: {0}")]
    ValidationError(String),

    #[error("Failed to parse numeric value for {key}: {source}")]
    ParseError {
        key: String,
        #[source]
        source: std::num::ParseIntError,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub leptos_output_name: String,
    pub leptos_site_root: String,
    pub leptos_site_pkg_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub environment: Environment,
    pub log_level: String,
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl std::str::FromStr for Environment {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "production" | "prod" => Ok(Environment::Production),
            "testing" | "test" => Ok(Environment::Testing),
            _ => Err(ConfigError::InvalidEnvVar {
                key: "ENVIRONMENT".to_string(),
                value: s.to_string(),
                reason: "must be one of: development, production, testing".to_string(),
            }),
        }
    }
}

// Global singleton instance
pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::load().expect("Failed to load configuration"));

impl Config {
    /// Load configuration from environment variables
    pub fn load() -> Result<Self, ConfigError> {
        // Load .env file if it exists (useful for development)
        dotenvy::dotenv().ok();

        let config = Config {
            server: ServerConfig {
                host: get_env_or_default("HOST", "127.0.0.1"),
                port: get_env_or_default("PORT", "3000").parse().map_err(|e| {
                    ConfigError::ParseError {
                        key: "PORT".to_string(),
                        source: e,
                    }
                })?,
                leptos_output_name: get_env_or_default("LEPTOS_OUTPUT_NAME", "leptos_start"),
                leptos_site_root: get_env_or_default("LEPTOS_SITE_ROOT", "target/site"),
                leptos_site_pkg_dir: get_env_or_default("LEPTOS_SITE_PKG_DIR", "pkg"),
            },
            database: DatabaseConfig {
                url: get_required_env("DATABASE_URL")?,
                max_connections: get_env_or_default("DATABASE_MAX_CONNECTIONS", "10")
                    .parse()
                    .map_err(|e| ConfigError::ParseError {
                        key: "DATABASE_MAX_CONNECTIONS".to_string(),
                        source: e,
                    })?,
            },
            app: AppConfig {
                environment: get_env_or_default("ENVIRONMENT", "development").parse()?,
                log_level: get_env_or_default("LOG_LEVEL", "info"),
                jwt_secret: get_required_env("JWT_SECRET")?,
            },
        };
        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate the configuration
    fn validate(&self) -> Result<(), ConfigError> {
        if self.database.url.is_empty() {
            return Err(ConfigError::ValidationError(
                "DATABASE_URL cannot be empty".to_string(),
            ));
        }

        if self.app.jwt_secret.len() < 32 {
            return Err(ConfigError::ValidationError(
                "JWT_SECRET must be at least 32 characters long".to_string(),
            ));
        }

        if self.server.port == 0 {
            return Err(ConfigError::ValidationError(
                "PORT must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Get a reference to the global config instance
    pub fn global() -> &'static Config {
        &CONFIG
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.app.environment == Environment::Development
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.app.environment == Environment::Production
    }

    /// Get the server bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

fn get_required_env(name: &str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingEnvVar(name.to_string()))
}

fn get_env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_parsing() {
        assert_eq!(
            "development".parse::<Environment>().expect("env missing"),
            Environment::Development
        );
        assert_eq!(
            "prod".parse::<Environment>().expect("env missing"),
            Environment::Production
        );
        assert_eq!(
            "test".parse::<Environment>().expect("env missing"),
            Environment::Testing
        );
        assert!("invalid".parse::<Environment>().is_err());
    }

    #[test]
    fn test_config_validation() {
        // This would require setting up test environment variables
        // You can expand these tests based on your needs
    }
}
