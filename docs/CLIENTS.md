# Clients

**Status: Proposed**

## Shared contract
Web UI and Windows Desktop Client consume the same versioned Control API. Business rules belong in the control plane, not duplicated in clients.

## Web UI
Candidate stack: React, TypeScript, Vite, Tailwind, shadcn/ui. It should present Dashboard, Projects, Servers, Deployments, Logs, Scheduler, Secrets, System, Security/Audit, and Settings according to authorization.

## Windows Desktop Client
Candidate stack: Tauri 2 + React + TypeScript. This is a proposal, not an accepted implementation decision. Tauri capabilities, updater model, local storage, signing, and Windows security constraints must be validated before ADR acceptance.

## Authentication
Clients must use a secure session/token mechanism defined by the authentication ADR. Tokens must not be exposed to untrusted page content or logs. Sensitive actions require server-side authorization and, where appropriate, explicit confirmation.

## Failure handling
Clients must treat API/Agent state as authoritative, show operation status, and never implement privileged fallback paths such as local arbitrary shell execution.
