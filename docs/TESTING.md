# Testing

**Status: Planned**

## Layers
- Unit: domain rules, schemas, adapters, versioning, authorization.
- Integration: database, Control API, Agent channel, adapters.
- API: authentication, authorization, validation, errors, idempotency.
- Agent: capability enforcement, path restrictions, privilege boundary.
- Security: command injection, path traversal, secret leakage, privilege escalation, unauthorized project/server access, replay/idempotency abuse, invalid deployment input.
- Deployment: backup, validation, activation, health checks, migration, rollback, partial failure.
- E2E: Web UI and Windows client against a test Control API.
- Windows client: packaging, signing/update behavior, secure credential handling.

## Critical negative tests
Prove that arbitrary command strings are rejected because no such API contract exists. Attempt path traversal and invalid registry references. Verify users cannot access projects/servers outside their scope. Inject secrets into error/log paths and assert masking.

## Reliability
Test Agent disconnects, API restart, scheduler restart, duplicate requests, deployment timeout, failed health check, failed rollback, corrupted/incompatible artifacts, and backup restore.

## Quality gates
Security-sensitive code requires automated regression tests before release. Production deployment is blocked when critical security tests fail.
