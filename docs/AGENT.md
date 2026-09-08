# HomeHub Agent

**Status: Proposed**

## Purpose
The Agent is the managed-server component that performs narrowly scoped privileged operations on behalf of the Control API.

## Boundary
```text
Control API -> authenticated local secure channel -> Agent -> allowlisted adapter -> resource
```

The API must never receive unrestricted root shell access. The Agent must not expose a generic command endpoint.

## Operation model
Examples of typed operations:
- project.start
- project.stop
- project.restart
- project.backup
- project.health_check
- project.update
- project.rollback
- infrastructure.health_check

Each operation has a fixed schema, authorization requirements, capability requirements, bounded parameters, timeout, result schema, and audit event.

## Validation
Validate server ID, project ID, registry state, adapter capability, canonical paths, version/artifact identity, and operation-specific constraints before execution. Reject path traversal and unknown resources.

## Communication
A local secure channel such as Unix socket is preferred where the API and Agent share a host. If the deployment topology requires another channel, use authenticated encryption and explicit server identity. The final mechanism is an ADR decision.

## Lifecycle
The Agent needs installation, registration, health reporting, controlled update, restart, rollback, and revocation procedures. Agent identity must be unique per managed server.

## Failure handling
Operations are bounded by timeouts and idempotency semantics where appropriate. Partial deployment state must be recorded. Failures must return safe structured errors and never leak credentials or raw command output containing secrets.
