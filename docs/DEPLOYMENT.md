# Deployment

**Status: Planned**

Deployment is a controlled state machine, not `git pull && docker compose up`.

```text
Release/artifact
  -> resolve desired version
  -> preflight + validate
  -> backup
  -> acquire deployment lock
  -> stage artifact
  -> build/install
  -> migrate if required
  -> activate/restart
  -> health check
  -> success
       or
     rollback
```

## State
Record desired version, installed version, deployment ID/status, server/project, started/finished timestamps, actor, artifact reference, backup reference, migration result, health result, and rollback information.

## Artifacts
Prefer GitHub Releases or immutable artifacts. Blind source pulls are not the official production deployment strategy. Artifact verification and compatibility checks are required before activation.

## Failure handling
A failed preflight must not mutate the live deployment. A failed activation or health check must preserve enough known-good state to roll back. Rollback itself is an auditable operation and has its own health verification.

## Safety
Deployment parameters are typed and registry-derived. No deployment API accepts arbitrary shell commands. Migrations must be versioned and reversible where technically possible.
