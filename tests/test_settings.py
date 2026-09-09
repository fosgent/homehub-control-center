"""Tests for the configuration/settings boundary."""

from __future__ import annotations

from homehub.core.config import Settings


def test_settings_apply_env_prefix_and_override(monkeypatch) -> None:
    monkeypatch.setenv("HOMEHUB_DATABASE_URL", "sqlite+aiosqlite:///./override.db")
    monkeypatch.setenv("HOMEHUB_LOG_LEVEL", "DEBUG")
    settings = Settings()
    assert settings.database_url == "sqlite+aiosqlite:///./override.db"
    assert settings.log_level == "DEBUG"


def test_settings_defaults_are_safe() -> None:
    settings = Settings()
    assert settings.api_v1_prefix == "/api/v1"
    assert settings.app_version == "0.1.0"
    # Bind to localhost by default; never to a wildcard interface.
    assert settings.host == "127.0.0.1"
