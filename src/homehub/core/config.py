"""Application configuration boundary.

Settings are loaded from environment variables (and optional .env files) and
never from committed source files. Secret values are represented as fields that
must be populated from the environment; no secrets are embedded in this
repository.
"""

from __future__ import annotations

from functools import lru_cache

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application configuration boundary.

    All values default to safe development defaults and may be overridden via
    environment variables prefixed with ``HOMEHUB_``. No secrets are committed;
    anything sensitive (e.g. future master-key material) must come from the
    environment or an external secret provider, never from source.
    """

    model_config = SettingsConfigDict(
        env_prefix="HOMEHUB_",
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )

    # Application
    app_name: str = "HomeHub Control Center"
    app_version: str = "0.1.0"
    api_v1_prefix: str = "/api/v1"
    debug: bool = False

    # Database (single-node MVP: SQLite + WAL, see ADR-009)
    database_url: str = "sqlite+aiosqlite:///./homehub.db"

    # Logging
    log_level: str = "INFO"

    # Server
    host: str = "127.0.0.1"
    port: int = 8000


@lru_cache
def get_settings() -> Settings:
    """Return the cached application settings instance."""
    return Settings()
