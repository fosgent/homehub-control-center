# ADR-008: Multi-server architecture

**Status:** Proposed

## Context
HomeHub starts with one server but should later manage additional HomeHub nodes and VPS hosts.

## Decision
Model the Control Plane and Managed Server as separate domains. Each server has a stable identity, Agent, capabilities, health/last-seen state, and authorization scope. Projects belong to a managed server.

## Alternatives
Single-server assumptions; direct client-to-server management; separate control plane per host.

## Consequences
Registration, revocation, connectivity state, capability negotiation, and cross-server authorization must be designed early.

## Security implications
A server identity is cryptographic and revocable. Disconnection is not authorization. Operations must be scoped to the selected server and its advertised capabilities.
