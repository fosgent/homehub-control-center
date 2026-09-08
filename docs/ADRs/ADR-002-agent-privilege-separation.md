# ADR-002: Agent privilege separation

**Status:** Accepted

## Context
Host operations may require privileges while the public Control API must not become an unrestricted host shell.

## Decision
Separate privileged execution into a server-side Agent. For the Phase 1 co-located topology, use HTTP/JSON over Unix Domain Socket at `/run/homehub/agent/agent.sock`. The Agent validates Linux peer identity, protocol version, typed operation schema, registry-derived resource references, capability, deadline, and idempotency state. It exposes only a finite allowlisted operation catalog.

The Agent runs as `homehub-agent`; the Control API runs as `homehub-api`. The socket directory is `homehub-agent:homehub-api` `0750`; the socket is `homehub-agent:homehub-api` `0660`. The Agent has no Internet/LAN/Tailnet listener.

## Alternatives
Run API as root; direct SSH; localhost TCP without process identity; generic privileged helper; gRPC for the initial local protocol. These were rejected for Phase 1 because they add trust surface or complexity without improving the required local boundary.

## Consequences
The Agent becomes a high-trust boundary and must be independently tested, audited, and recoverable. API compromise does not automatically become unrestricted host execution because the Agent contract remains finite and resource-scoped.

## Security implications
Socket permissions are a transport gate, not the complete authorization model. Capability never equals user authorization. No arbitrary command strings, executable paths, or shell passthrough are permitted.
