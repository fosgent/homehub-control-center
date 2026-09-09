"""Health reporting use case and the database probe port.

The application service depends only on the ``DatabaseProbe`` port, never on a
concrete engine or driver. This keeps the application layer SQLite-independent:
any conforming probe (SQLite, PostgreSQL, or a stub in tests) can be supplied.
"""

from __future__ import annotations

from typing import Protocol

from homehub.domain.health import SystemHealth


class DatabaseProbe(Protocol):
    """Port for checking database connectivity.

    Implemented by an infrastructure adapter. Returns ``True`` when the database
    is reachable and a basic connectivity check succeeds.
    """

    async def is_healthy(self) -> bool: ...


class HealthService:
    """Application service that composes a SystemHealth snapshot.

    The status is derived purely from domain rules: if the database probe fails,
    the plane reports ``degraded``. A transport-level response must not be
    treated as durable truth; callers rely on the returned domain object.
    """

    def __init__(self, database_probe: DatabaseProbe, version: str) -> None:
        self._database_probe = database_probe
        self._version = version

    async def check(self) -> SystemHealth:
        db_ok = False
        try:
            db_ok = await self._database_probe.is_healthy()
        except Exception:
            db_ok = False

        database = "ok" if db_ok else "failed"
        status = "ok" if db_ok else "degraded"
        return SystemHealth(
            status=status,
            database=database,
            version=self._version,
            checks=(("database", database),),
        )
