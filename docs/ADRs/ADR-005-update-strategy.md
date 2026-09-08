# ADR-005: Update strategy

**Status:** Accepted

## Context
Control Center and managed projects need predictable version discovery, artifact integrity, and recovery.

## Decision
Use exact immutable release/artifact references. Production prefers signed artifacts/provenance with pinned trust roots. MVP additionally supports exact SHA-256 digest verification where signing is unavailable. Repository identity is pinned. Verification failure, repository mismatch, incompatible artifact, or mutable branch reference fails closed.

## Alternatives
Blind `git pull`; downloading mutable branches; manual updates only.

## Consequences
Release publication becomes part of operational discipline. Verification and recovery are required before production self-update.

## Security implications
Unverified artifacts never activate. Known-good rollback material is preserved before activation.
