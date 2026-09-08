# Operations

**Status: Planned runbook**

## Deploy
1. Confirm authorized project/server.
2. Check current health and deployment lock.
3. Create and verify backup.
4. Stage and validate the release.
5. Deploy through the registered adapter.
6. Run migrations if required.
7. Activate/restart.
8. Run health checks.
9. Record success or execute rollback.

## Update
Use the same controlled deployment lifecycle. Never use an unreviewed blind `git pull` as the production procedure.

## Rollback
Identify the known-good version and backup, acquire the deployment lock, restore/deploy through the adapter, verify health, and record the result.

## Backup/restore
Backups require integrity checks, retention policy, protected storage, and restore testing. Restore operations are privileged and audited.

## Restart/health
Restart only through typed project/infrastructure operations. Health checks must distinguish process availability, application health, dependencies, and host health.

## Incident handling
Preserve audit and relevant logs, identify affected server/project, contain through controlled stop/disable operations where safe, recover from known-good artifacts/backups, verify health, then document the incident.
