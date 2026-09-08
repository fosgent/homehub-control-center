# Testing

**Status: Accepted quality-gate contract**

## Architecture/security tests
- RBAC matrix: every listed operation tested for allow/deny and scope.
- Authentication: session expiry, logout revocation, password reset, admin recovery, refresh rotation, refresh reuse detection.
- CSRF/CORS/CSP/cookie attributes and host validation.
- Agent peer authentication and protocol-version rejection.
- Capability vs authorization separation.
- Request schema, stable error codes, deadlines, idempotency and duplicate handling.
- Path traversal, absolute paths, NUL bytes, encoded traversal, symlink/magic-link escape, cross-project references, and TOCTOU-sensitive operations.
- Secret masking in errors/logs/audit and failure when key material is unavailable.
- Artifact signature/digest verification failure must fail closed.
- Server A credentials/identity cannot authorize Server B.

## Deployment/restore tests
- backup failure blocks destructive deployment;
- artifact verification failure leaves live state untouched;
- activation/health failure reaches deterministic state;
- rollback succeeds to explicit known-good point;
- rollback failure reaches `rollback_failed`/`unknown_recovery_required`;
- restore verifies backup integrity and target compatibility before mutation;
- disk-full and database-corruption paths are safe and audited.

## Reliability
Test API restart, Agent disconnect, duplicate requests, timeout with unknown outcome, scheduler restart, expired credentials, stale server, and recovery after reboot.

## Phase 1 gate
All security-critical contract tests must exist before Phase 1 is declared complete. Documentation readiness alone never substitutes for executable tests once implementation begins.
