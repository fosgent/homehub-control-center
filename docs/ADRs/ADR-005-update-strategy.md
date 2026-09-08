# ADR-005: Update strategy

**Status:** Proposed

## Context
Control Center and managed projects need predictable version discovery and recovery.

## Decision
Prefer GitHub Releases or immutable artifacts, verify identity/integrity, stage before activation, and use health checks with rollback for managed projects. Treat Control Center self-update as a separate lifecycle.

## Alternatives
Blind `git pull`; downloading mutable branches; manual updates only.

## Consequences
Release publication becomes part of operational discipline. Verification and recovery must be implemented before production self-update.

## Security implications
Trust roots and verification failures must fail closed. Unverified artifacts must never be activated.
