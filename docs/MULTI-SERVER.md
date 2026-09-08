# Multi-Server

**Status: Planned**

The architecture separates the Control Plane from Managed Servers from the beginning.

```text
Control Plane
 ├── HomeHub A -> Agent
 ├── HomeHub B -> Agent
 └── Future VPS -> Agent
```

Each server has a stable Server ID, display name, network/address metadata, Agent identity/status, Agent version, OS, capabilities, health, and last-seen timestamp.

## Registration
Server registration must establish a unique cryptographic identity, bind the Agent to an authorized server record, and support revocation. Registration is not proof of operator authorization by itself.

## Capabilities
Agents advertise a bounded capability set such as Docker, Nginx, filesystem paths, backups, or health checks. The Control Plane must authorize against both requested operation and server capability.

## Communication
Prefer private Tailscale connectivity and authenticated encrypted transport. Agent-to-control authentication and server identity must be explicit; the final protocol remains a proposed ADR decision.

## Failure
A disconnected server is unavailable, not implicitly authorized. Last-seen state is clearly distinguished from current health. Commands are not queued indefinitely without expiration and idempotency rules.
