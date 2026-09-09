"""Domain model for system health reporting.

Defined as pure domain types with no framework or storage coupling so they can
be reused across the Control API, Agent, and future clients without leaking
infrastructure concerns.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone


@dataclass(frozen=True)
class SystemHealth:
    """A snapshot of Control Plane health at a point in time.

    ``status`` is intentionally coarse and stable so that both observers and
    automation can treat it as durable truth rather than guessing from a
    transport-level response. ``checks`` lists individual component results.
    """

    status: str  # one of: ok, degraded, unavailable
    database: str  # one of: ok, failed, not_checked
    version: str
    timestamp: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    checks: tuple[tuple[str, str], ...] = ()
