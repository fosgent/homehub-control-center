# HomeHub Agent

**Status: Accepted implementation contract for Phase 1**

## Purpose
The Agent is the managed-server privileged boundary. It performs only named, typed operations authorized by the Control Plane and supported by registered host capabilities.

## Transport and identity
Co-located Control API and Agent communicate using HTTP/JSON over Unix Domain Socket:

```text
/run/homehub/agent/agent.sock
```

Runtime directory: `/run/homehub/agent`, owner `homehub-agent`, group `homehub-api`, mode `0750`. Socket owner `homehub-agent`, group `homehub-api`, mode `0660`. Control API runs as `homehub-api`; Agent runs as `homehub-agent`. The Agent never listens on TCP, Internet, LAN, or Tailnet interfaces.

Socket permissions are only the first access gate. The Agent also checks Linux peer credentials (`SO_PEERCRED`) for the exact API service identity, protocol version, server ID, request shape, deadline, and operation allowlist. User RBAC is decided by the Control API; Agent capability is a second, independent boundary and never grants user authorization.

Remote Agents are deferred. When introduced, they use mutually authenticated TLS over private Tailscale connectivity with explicit server identity and certificate revocation.

## Capability catalog
Capabilities are finite and named, for example:
- `project.start`
- `project.stop`
- `project.restart`
- `project.pause`
- `project.resume`
- `project.deploy`
- `project.rollback`
- `project.backup`
- `project.restore`
- `project.health_check`
- `infrastructure.health_check`
- `nginx.site_enable`
- `nginx.site_disable`
- `system.service_restart`

Each capability defines request schema, allowed resource kinds, required registry state, timeout, idempotency class, result schema, and audit semantics. There is no `execute`, `shell`, `command`, arbitrary executable, or free-form subprocess capability.

## Request contract
`AgentRequest` contains:
- `protocol_version`: negotiated protocol version;
- `request_id`: UUID identifying one logical API request;
- `idempotency_key`: client/application retry key for mutating operations;
- `server_id`: expected managed-server identity;
- `operation`: registered capability name;
- `parameters`: operation-specific typed object;
- `deadline_at`: absolute UTC deadline;
- `authorization_context`: non-secret actor/decision metadata for audit correlation, not a substitute for API authorization.

`AgentResponse` contains:
- `protocol_version`;
- `request_id`;
- `status`: `accepted | succeeded | failed | rejected | expired | unknown`;
- `result`: typed operation result or null;
- `error`: structured error or null;
- `retryable`: boolean;
- `duration_ms`;
- `operation_state_id` for asynchronous/long-running operations where applicable.

Stable machine-readable error codes include `AUTHENTICATION_FAILED`, `AUTHORIZATION_CONTEXT_INVALID`, `UNKNOWN_OPERATION`, `UNSUPPORTED_VERSION`, `INVALID_PARAMETERS`, `RESOURCE_NOT_FOUND`, `RESOURCE_SCOPE_VIOLATION`, `CAPABILITY_UNAVAILABLE`, `DEADLINE_EXCEEDED`, `DUPLICATE_REQUEST`, `IDEMPOTENCY_CONFLICT`, `OPERATION_IN_PROGRESS`, `VERIFICATION_FAILED`, `PRECONDITION_FAILED`, `EXECUTION_FAILED`, `ROLLBACK_FAILED`, and `INTERNAL_ERROR`. Client-facing messages are sanitized; stack traces and secrets never cross the boundary.

## Idempotency
Mutating requests carry an idempotency key. The Agent/API persist operation state for at least 24 hours for completed requests and for the lifetime of in-progress operations. A repeated key with the same operation and parameters returns the original operation state/result. Reuse with different parameters is `IDEMPOTENCY_CONFLICT`.

`start`, `stop`, `pause`, `resume`, and `health_check` are state-convergent and safe to retry when the prior outcome is known or the operation state is still owned. `restart`, `backup`, `deploy`, `rollback`, and `restore` are operation-tracked; retry after an unknown outcome first queries the operation state and current resource state. A new idempotency key means a new operation and is never treated as a retry automatically.

## Validation and filesystem boundary
Agent operations reference `project_id` plus a typed resource kind (`app`, `data`, `config`, `backup`) and, only where needed, a validated relative subpath. The Agent resolves roots from the current registry snapshot; clients cannot supply replacement roots.

Sensitive file operations reject absolute paths, NUL bytes, `.`/`..` traversal components, encoded traversal after a single canonical decode, path separators that escape the resource grammar, symlink/magic-link traversal, and roots belonging to another project. Use descriptor-relative operations and atomic temp-file + rename patterns for writes. Linux `openat2()` is the preferred primitive where available because `RESOLVE_BENEATH` and `RESOLVE_NO_SYMLINKS` provide kernel-enforced resolution constraints.

## Failure
The Agent fails closed on authentication failure, invalid capability, registry mismatch, verification failure, expired deadline, missing required backup, or unsafe filesystem resolution. Every terminal operation has a deterministic state and audit result. Partial deployment/restore state is persisted so recovery can choose a known-good point rather than guessing.
