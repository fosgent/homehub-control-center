# Updates

**Status: Accepted update-security contract**

## Control Center self-update
```text
discover release -> resolve exact version -> acquire immutable artifact -> verify identity/integrity/signature -> stage -> compatibility check -> install -> health check -> recover/rollback
```

Self-update never executes arbitrary repository content and never treats a mutable branch as a trusted artifact.

## Project update
Project updates use the Deployment state machine and produce deployment plus audit records.

## Trust policy
The repository owner/name and expected artifact family are pinned in configuration/registry. Production prefers signed artifacts with a pinned verification trust root and provenance. Where signing is not yet available, the MVP requires an exact SHA-256 digest recorded before activation. A mismatch, missing digest, invalid signature, unexpected repository, incompatible version, or expired artifact is a hard failure.

## Versioning
Desired and installed versions are stored separately and compared with one normalized versioning rule set. Artifact identity is never inferred from a human-readable version string alone.

## Recovery
Before activation, the current known-good state and rollback material are recorded. Failed update verification does not touch the live version. Failed activation/health check enters deterministic failure/rollback state.
