"""Shared test fixtures and application wiring.

Tests run against an in-memory SQLite database so they are hermetic and never
write to a development or production database file.
"""

from __future__ import annotations

from collections.abc import AsyncIterator

import pytest
import pytest_asyncio
from httpx import ASGITransport, AsyncClient

from homehub.core.config import Settings
from homehub.main import create_app


@pytest.fixture
def app_settings() -> Settings:
    """Isolated settings pointing at an in-memory database."""
    return Settings(
        database_url="sqlite+aiosqlite:///:memory:",
        log_level="ERROR",
    )


@pytest.fixture
def app(app_settings: Settings):
    """Build the FastAPI application over an in-memory database."""
    return create_app(app_settings)


@pytest_asyncio.fixture
async def client(app) -> AsyncIterator[AsyncClient]:
    """Provide an ASGI test client for the application."""
    transport = ASGITransport(app=app)
    async with AsyncClient(transport=transport, base_url="http://test") as c:
        yield c
