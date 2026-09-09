"""FastAPI application factory.

Composition root: wires the API/interface layer to the application and
infrastructure layers and configures routing. It keeps the domain/application
layers free of HTTP concerns and the endpoint code free of persistence details.
"""

from __future__ import annotations

import logging

from fastapi import FastAPI

from homehub.api.router import api_router
from homehub.core.config import Settings, get_settings
from homehub.infrastructure.session import init_engine


def configure_logging(settings: Settings) -> None:
    logging.basicConfig(
        level=getattr(logging, settings.log_level.upper(), logging.INFO),
        format="%(levelname)s [%(name)s] %(message)s",
    )


def create_app(settings: Settings | None = None) -> FastAPI:
    """Create and configure the FastAPI application.

    ``settings`` may be injected for tests; when omitted the cached settings are
    used. The database engine is (re)initialised here, binding the configured
    persistence URL.
    """
    settings = settings or get_settings()
    configure_logging(settings)

    init_engine(settings.database_url)

    app = FastAPI(
        title=settings.app_name,
        version=settings.app_version,
        debug=settings.debug,
    )
    app.include_router(api_router, prefix=settings.api_v1_prefix)

    @app.get("/", include_in_schema=False)
    async def root() -> dict[str, object]:
        return {"name": settings.app_name, "version": settings.app_version}

    return app


app = create_app()
