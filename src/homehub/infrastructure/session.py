"""SQLAlchemy engine and session factory for the Control Plane database.

Owns the single-node MVP engine (SQLite + aiosqlite, WAL mode per ADR-009).
The engine is created lazily from the application settings. Nothing above the
infrastructure layer constructs the engine or sessions directly.
"""

from __future__ import annotations

from collections.abc import AsyncIterator

from sqlalchemy.ext.asyncio import (
    AsyncEngine,
    AsyncSession,
    async_sessionmaker,
    create_async_engine,
)
from sqlalchemy.pool import NullPool

_ENGINE: AsyncEngine | None = None
_SESSION_FACTORY: async_sessionmaker[AsyncSession] | None = None


def create_engine(database_url: str) -> AsyncEngine:
    """Create a configured async engine for the given database URL.

    The single-node MVP targets a single Control API instance on one host
    (ADR-009), so we do not attempt horizontal scaling. SQLite connections are
    not pooled because sharing a single SQLite connection across async tasks is
    unsafe; the engine opens fresh connections per operation.
    """
    kwargs: dict[str, object] = {"poolclass": NullPool}
    return create_async_engine(database_url, **kwargs)


def init_engine(database_url: str) -> AsyncEngine:
    """Initialise the application-wide engine and session factory.

    Returns the engine and stores it for dependency injection. Calling this
    again replaces the engine (used in tests to swap in an in-memory database).
    """
    global _ENGINE, _SESSION_FACTORY
    _ENGINE = create_engine(database_url)
    _SESSION_FACTORY = async_sessionmaker(_ENGINE, expire_on_commit=False)
    return _ENGINE


def get_session_factory() -> async_sessionmaker[AsyncSession]:
    """Return the application-wide session factory.

    Raises ``RuntimeError`` if the engine has not been initialised.
    """
    if _SESSION_FACTORY is None:
        raise RuntimeError("Database engine has not been initialised")
    return _SESSION_FACTORY


async def get_session() -> AsyncIterator[AsyncSession]:
    """FastAPI dependency that yields an :class:`AsyncSession`."""
    async with get_session_factory()() as session:
        yield session
