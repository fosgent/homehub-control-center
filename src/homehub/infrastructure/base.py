"""SQLAlchemy declarative base and shared metadata.

SQLite is the single-node MVP persistence engine (ADR-009). All ORM models
inherit from ``Base`` so Alembic can autogenerate against a single metadata
object. Domain/application layers never touch ``Base`` or SQLAlchemy types.
"""

from __future__ import annotations

from sqlalchemy.orm import DeclarativeBase


class Base(DeclarativeBase):
    """Declarative base for all persistence models."""


metadata = Base.metadata
