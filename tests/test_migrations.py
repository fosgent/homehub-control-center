"""Tests for the Alembic migration infrastructure.

Verifies the migration chain applies and can be rolled back against a real
(temp-file) SQLite database, confirming the wiring between alembic.ini, the
application settings boundary, and the declarative metadata is correct.
"""

from __future__ import annotations

from pathlib import Path

from alembic import command
from alembic.config import Config


def _alembic_config(db_path: Path) -> Config:
    cfg = Config(str(Path("alembic.ini")))
    cfg.set_main_option("sqlalchemy.url", f"sqlite+aiosqlite:///{db_path}")
    return cfg


def test_alembic_upgrade_and_downgrade(tmp_path: Path) -> None:
    db_path = tmp_path / "migration_test.db"

    cfg = _alembic_config(db_path)

    command.upgrade(cfg, "head")
    assert _heads(db_path) == "0001"

    command.downgrade(cfg, "base")
    # No revision should remain applied after downgrading to base.
    assert _heads(db_path) == ""


def _heads(db_path: Path) -> str:
    import sqlite3

    con = sqlite3.connect(db_path)
    try:
        row = con.execute("SELECT version_num FROM alembic_version").fetchone()
        return row[0] if row else ""
    finally:
        con.close()
