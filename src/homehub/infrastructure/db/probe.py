"""SQLAlchemy-based database probe adapter.

Implements the application-layer ``DatabaseProbe`` port using the configured
async engine. Uses a real, bounded connectivity check (a trivial ``SELECT 1``)
through the session factory so it exercises the full persistence path.
"""

from __future__ import annotations

from sqlalchemy import text

from homehub.application.health import DatabaseProbe
from homehub.infrastructure.session import get_session_factory


class SqlAlchemyDatabaseProbe(DatabaseProbe):
    """Probe that verifies live database connectivity via ``SELECT 1``."""

    async def is_healthy(self) -> bool:
        try:
            async with get_session_factory()() as session:
                await session.execute(text("SELECT 1"))
                return True
        except Exception:
            return False
