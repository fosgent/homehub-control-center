"""Domain layer.

The domain layer contains business concepts and rules independent of any
framework or persistence technology. It must not depend on SQLite, SQLAlchemy,
FastAPI, or any infrastructure concern (per ADR-009). Infrastructure adapters
implement the ports defined here.
"""

from __future__ import annotations

from .health import SystemHealth

__all__ = ["SystemHealth"]
