"""Health endpoint.

Exposes the application health use case over HTTP. The handler only formats the
domain result; the actual health decision lives in the application service.
The database probe is resolved through a dependency so tests can substitute a
stub without touching persistence.
"""

from __future__ import annotations

from fastapi import APIRouter, Depends

from homehub.application.health import DatabaseProbe, HealthService
from homehub.core.config import get_settings
from homehub.infrastructure.db.probe import SqlAlchemyDatabaseProbe

router = APIRouter(tags=["health"])


def get_database_probe() -> DatabaseProbe:
    """Resolve the database probe used by the health service.

    Overridable in tests via ``app.dependency_overrides``.
    """
    return SqlAlchemyDatabaseProbe()


def get_health_service(probe: DatabaseProbe = Depends(get_database_probe)) -> HealthService:
    settings = get_settings()
    return HealthService(database_probe=probe, version=settings.app_version)


@router.get("/health")
async def get_health(service: HealthService = Depends(get_health_service)) -> dict[str, object]:
    """Return the Control Plane health snapshot.

    The response reflects domain-computed health (e.g. ``degraded`` when the
    database is unreachable) rather than assuming success from the HTTP status.
    """
    health = await service.check()
    return {
        "status": health.status,
        "database": health.database,
        "version": health.version,
        "timestamp": health.timestamp.isoformat(),
        "checks": [{"name": name, "status": status} for name, status in health.checks],
    }
