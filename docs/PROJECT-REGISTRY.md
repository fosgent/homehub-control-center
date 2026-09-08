# Project Registry

**Status: Proposed**

The registry is the canonical inventory of projects managed by the Control Plane. It must be extensible and must not encode assumptions specific to one application.

Example:
```yaml
project:
  id: qanetix
  name: Qanetix
  repository:
    provider: github
    repository: fosgent/qanetix
  deployment:
    type: static
    web_root: /srv/apps/qanetix
  runtime:
    type: nginx
  healthcheck:
    url: /
  paths:
    app: /srv/apps/qanetix
    data: /srv/data/qanetix
    config: /srv/configs/qanetix
    backup: /srv/backups/qanetix
```

## Required concepts
ID, display name, server ID, repository/artifact source, deployment type, runtime, health checks, capability set, canonical paths, desired version, installed version, deployment state, backup references, and lifecycle status.

## Safety
Registry values are data, not shell templates. Paths must be canonicalized and validated against managed roots. Runtime/deployment types select registered adapters. Unknown adapter types and unbounded path overrides are rejected.

## Lifecycle
Projects may be registered, validated, enabled/disabled, updated, backed up, rolled back, and removed. Removal must define data/config/backup retention explicitly and require elevated confirmation.
