"""API / interface layer.

FastAPI routers and HTTP-facing schemas. This layer translates HTTP into calls
to application services and formats responses. It contains no domain logic and
no persistence details; it depends on the application layer only.
"""

from __future__ import annotations
