# ADR-003: Project Registry

**Status:** Proposed

## Context
Projects differ in runtime, deployment, paths, repositories, and health checks. Hard-coding a single project would prevent a reusable control plane.

## Decision
Create a canonical, extensible Project Registry containing identity, server scope, source, deployment/runtime type, capabilities, health checks, canonical managed paths, versions, and lifecycle state.

## Alternatives
Per-project code; static shell scripts; unstructured YAML only.

## Consequences
Adapters can operate from validated registry data. Schema evolution and migrations become first-class concerns.

## Security implications
Registry data is untrusted configuration. It must not become shell templates; paths and operation types are validated and allowlisted.
