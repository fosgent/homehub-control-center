# Product

**Status: Proposed foundation**

## Problem
HomeHub hosts multiple applications and infrastructure components. Operating them independently through SSH, ad-hoc scripts, and project-specific tooling creates fragmented visibility, inconsistent deployment procedures, and excessive privilege.

## Vision
HomeHub Control Center is a central control plane for managing HomeHub and future managed servers. It provides one authenticated interface and one API for inventory, health, operations, deployments, updates, scheduling, logs, secrets, security, and audit.

## Target users
- HomeHub administrator/operator.
- Future trusted operators with role-based access.

## Core use cases
- See server and project health.
- Start, stop, restart, update, back up, and health-check managed projects.
- Track deployments and rollbacks.
- Inspect logs and audit events.
- Manage scheduled jobs and protected secrets.
- Add additional managed servers.

## Non-goals
- Generic remote shell.
- Arbitrary command execution through the API.
- Replacing Docker, Nginx, Tailscale, GitHub, or the operating system.
- Giving the Web UI direct privileged access to the host.

## MVP direction
1. Control API + Agent security boundary.
2. Authentication/authorization.
3. Project Registry.
4. Server health and controlled project operations.
5. Audit logging.
6. Web/desktop clients consuming the same API.

## Future
Deployment orchestration, scheduler, secrets management, self-update, multi-server management, richer observability, and notification workflows.
