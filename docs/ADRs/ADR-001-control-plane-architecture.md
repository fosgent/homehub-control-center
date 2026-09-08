# ADR-001: Control-plane architecture

**Status:** Proposed

## Context
HomeHub needs centralized management across projects and eventually multiple servers. Clients must not contain privileged business logic.

## Decision
Use a Control Plane composed of a shared API, persistence/domain services, and a server-side Agent boundary. Web and Windows clients consume the same API. Managed-server operations are delegated to typed adapters.

## Alternatives
Direct SSH from clients; dashboard-only architecture; API with unrestricted shell access.

## Consequences
Centralized authorization and audit become possible. The Agent adds a component and deployment lifecycle but materially reduces privilege exposure.

## Security implications
No arbitrary command API. Privileged actions are typed, validated, capability-scoped, and audited.
