"""Convenience re-export of the configuration boundary.

Public import point: ``from homehub.settings import get_settings, Settings``.
"""

from __future__ import annotations

from .core.config import Settings, get_settings

__all__ = ["Settings", "get_settings"]
