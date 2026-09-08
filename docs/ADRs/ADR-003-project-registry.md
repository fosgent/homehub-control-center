# ADR-003: Project Registry

**Status:** Accepted

## Context
Projects differ in runtime, deployment, paths, repositories, health checks, and capabilities. A reusable control plane requires a canonical inventory.

## Decision
Persist a versioned Project Registry in the Control Plane database. It is the authoritative source for project identity, server assignment, artifact source, deployment/runtime adapter, capabilities, health checks, canonical filesystem roots, desired/installed versions, lifecycle state, and backup references. YAML is data-only import/export, never an executable source of commands.

Filesystem roots are registry-derived from server-managed root policies. Operation requests may identify a project and typed resource/subpath but cannot replace the root. Registry mutations require admin authorization, schema validation, optimistic concurrency, and audit.

## Alternatives
Per-project code; static shell scripts; unstructured YAML as runtime authority.

## Consequences
Adapters become reusable and schema migrations are first-class. Registry corruption or malicious mutation becomes a security-sensitive configuration event.

## Security implications
Registry data cannot become shell templates. Unknown adapter/capability types, unbounded path overrides, cross-project references, and invalid lifecycle transitions are rejected.
