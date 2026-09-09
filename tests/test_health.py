"""Tests for the health reporting use case and endpoint."""

from __future__ import annotations

from homehub.api.health import get_database_probe
from homehub.application.health import DatabaseProbe, HealthService


class FakeProbe(DatabaseProbe):
    def __init__(self, healthy: bool = True) -> None:
        self._healthy = healthy

    async def is_healthy(self) -> bool:
        return self._healthy


class FailingProbe(DatabaseProbe):
    async def is_healthy(self) -> bool:
        raise RuntimeError("boom")


async def test_health_service_reports_ok_when_database_healthy() -> None:
    service = HealthService(database_probe=FakeProbe(healthy=True), version="0.1.0")
    health = await service.check()
    assert health.status == "ok"
    assert health.database == "ok"
    assert dict(health.checks) == {"database": "ok"}


async def test_health_service_reports_degraded_when_database_down() -> None:
    service = HealthService(database_probe=FakeProbe(healthy=False), version="0.1.0")
    health = await service.check()
    assert health.status == "degraded"
    assert health.database == "failed"


async def test_health_service_degrades_on_probe_exception() -> None:
    service = HealthService(database_probe=FailingProbe(), version="0.1.0")
    health = await service.check()
    assert health.status == "degraded"
    assert health.database == "failed"


async def test_health_endpoint_returns_ok(client) -> None:
    resp = await client.get("/api/v1/health")
    assert resp.status_code == 200
    body = resp.json()
    assert body["status"] == "ok"
    assert body["database"] == "ok"
    assert body["version"] == "0.1.0"
    assert "timestamp" in body


async def test_health_endpoint_degrades_when_probe_fails(client, app) -> None:
    async def override() -> DatabaseProbe:
        return FakeProbe(healthy=False)

    app.dependency_overrides[get_database_probe] = override
    try:
        resp = await client.get("/api/v1/health")
        assert resp.status_code == 200
        assert resp.json()["status"] == "degraded"
        assert resp.json()["database"] == "failed"
    finally:
        app.dependency_overrides.clear()


async def test_health_endpoint_is_under_api_v1_prefix(client) -> None:
    resp = await client.get("/api/v1/health")
    assert resp.status_code == 200
