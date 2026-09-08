# Deployment

**Status: Accepted lifecycle contract**

Deployment is a durable state machine, not `git pull && docker compose up`.

```text
Request
 -> authorize
 -> resolve project/server
 -> preflight
 -> acquire lock
 -> create + verify backup
 -> acquire exact artifact
 -> verify identity/integrity
 -> stage
 -> compatibility/migration prechecks
 -> activate/restart
 -> health check
 -> SUCCESS
      or
   deterministic FAILURE / ROLLBACK
```

## Deployment states
Minimum states: `requested`, `authorized`, `preflight_failed`, `backup_failed`, `artifact_failed`, `staging`, `activating`, `health_checking`, `succeeded`, `failed`, `rollback_pending`, `rolling_back`, `rolled_back`, `rollback_failed`, `unknown_recovery_required`.

A terminal state must explain whether the live deployment is known-good, known-bad, rolled back, or requires operator recovery. The system must never report success when activation/health status is unknown.

## Preconditions
- actor authorized for project/server;
- no conflicting deployment lock;
- registry snapshot valid;
- disk capacity sufficient where measurable;
- exact artifact reference available;
- artifact identity/integrity verified;
- backup policy satisfied.

A failed required backup blocks destructive deployment. An explicit project policy may declare backup unnecessary only for non-destructive operations and must itself be admin-controlled.

## Artifact policy
Production deployments use an exact immutable release/artifact reference. Repository identity is pinned in the registry. Signed artifacts/provenance are preferred. MVP requires at minimum exact SHA-256 digest verification for any artifact lacking a trusted signature. Mutable branch heads are not deployable production artifacts. Verification failure fails closed.

## Rollback
Every deployment records a rollback point consisting of the known-good artifact/version plus required backup/state references. Rollback is itself a tracked operation with lock, verification, activation, health check, and audit. If rollback fails, state becomes `rollback_failed` or `unknown_recovery_required`; the system does not silently retry destructive actions.

## Backup acceptance gate
A deployment backup is valid only when the declared backup set completed, metadata was durably persisted, integrity verification passed, permissions/ownership metadata needed for restore was captured, and a restore procedure can locate the backup and identify its compatibility with the target project/version.

## Idempotency
Deployment uses a durable idempotency key and deployment ID. A client retry first resolves the existing deployment state. A new deployment requires a new key and passes all preconditions again.
