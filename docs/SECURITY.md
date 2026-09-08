# Security

**Status: Proposed**

## Security objectives
Protect the control plane, managed servers, project data, credentials, and operational history while keeping privileged operations narrowly scoped and auditable.

## Threat model
Threats include stolen sessions, unauthorized operators, API abuse, path traversal, command injection, secret leakage, compromised project artifacts, malicious update input, Agent compromise, privilege escalation, replayed operations, and destructive operator mistakes.

## Trust and authorization
Authentication is required before sensitive resources are exposed. Authorization is deny-by-default and must be evaluated server-side for every protected operation. RBAC is planned. Server and project scope must be part of authorization decisions.

## Agent security
The API must not receive unrestricted root shell access. The Agent exposes typed operations only. Each operation validates identity, capability, resource scope, paths, versions, and bounded parameters. Agent credentials and local IPC permissions must be least-privilege.

## Forbidden interface
Never implement `POST /execute`, command strings, arbitrary subprocess parameters, or equivalent shell passthrough. New privileged operations require a named adapter contract and audit semantics.

## Secrets
Secrets are never committed to Git, rendered in plaintext, placed in ordinary configuration, or written to logs. Storage must use authenticated encryption at rest; key management, rotation, access policy, and recovery must be documented before production use.

## Network
Prefer private Tailscale connectivity plus HTTPS and application authentication. Tailscale Funnel is not the production trust model. TLS termination and certificate handling must be explicit.

## Audit
Authentication failures, privileged project actions, deployments, rollbacks, backups, secret access/change, configuration changes, role changes, and server registration must produce immutable or tamper-evident audit records with actor, target, timestamp, correlation ID, result, and safe metadata.

## Update integrity
Prefer signed/verified GitHub Releases or immutable artifacts over blind `git pull`. Validate artifact identity and compatibility before installation; health-check after activation and preserve rollback material.

## Security testing
Tests must include command injection, path traversal, unauthorized project/server access, secret leakage, privilege escalation, replay/idempotency abuse, invalid deployment input, and rollback failure paths.
