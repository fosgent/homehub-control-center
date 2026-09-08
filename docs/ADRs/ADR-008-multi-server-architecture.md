# ADR-008: Multi-server architecture

**Status:** Accepted

## Context
HomeHub starts with one server but must later manage additional HomeHub nodes and VPS hosts without creating implicit cross-server trust.

## Decision
Model Control Plane and Managed Server as separate domains. Each server has a stable `server_id`, unique cryptographic Agent identity, capabilities, health/last-seen state, and explicit authorization scope. Registration and revocation are admin-only. Server capabilities constrain Agent execution but do not grant user authorization.

Phase 1 uses local UDS. Future remote Agents use mTLS over private Tailscale connectivity with certificate identity bound to `server_id`.

## Alternatives
Single-server assumptions; direct client-to-server management; separate control plane per host; network identity as authorization.

## Consequences
Enrollment, revocation, stale/offline state, capability negotiation, and cross-server authorization are explicit design concerns.

## Security implications
Server A compromise does not authorize Server B. Revocation prevents new privileged work. Re-enrollment after compromise creates a new Agent identity.
