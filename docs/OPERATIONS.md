# Operations

**Status: Accepted operational runbook contract**

## Deploy
1. Confirm actor and project/server authorization.
2. Check current health and deployment lock.
3. Run preflight checks.
4. Create and verify the required backup.
5. Acquire and verify the exact immutable artifact.
6. Stage and validate.
7. Run migrations if required.
8. Activate/restart.
9. Run health checks.
10. Record deterministic success or execute the recorded rollback point.

## Retry and unknown outcome
Never interpret an HTTP timeout as proof that the operation did not run. Query the durable operation/deployment state using its ID/idempotency key. Only start a new operation after confirming that the prior operation is terminal and that a new operation is actually required.

## Backup
Backups are considered usable only after successful creation, integrity verification, durable metadata persistence, and compatibility/restore identification. Restore is admin-only, audited, and must validate the target before mutation.

## Restore acceptance procedure
Select backup -> verify manifest/integrity -> validate target project/server and available capacity -> acquire lock -> restore files/data/config through typed adapters -> restore required ownership/permissions -> run health checks -> verify expected version/state -> record audit result.

## Rollback
Rollback targets an explicit known-good artifact/version plus required backup/state references. If rollback cannot complete safely, mark the deployment `rollback_failed` or `unknown_recovery_required` and stop further destructive automation.

## Incident handling
For Agent compromise or host compromise: revoke server identity, isolate the host, preserve audit/log evidence, rotate affected credentials, reinstall/recover from trusted artifacts/backups, and re-enroll with a new identity.
