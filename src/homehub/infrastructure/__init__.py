"""Infrastructure layer.

Concrete implementations of application ports, including persistence adapters
built on SQLAlchemy/SQLite (per ADR-009). Nothing above this layer imports these
modules directly; they are wired in at application composition.
"""

from __future__ import annotations
