# ADR-007: Secrets management

**Status:** Proposed

## Context
Managed projects require credentials, but the control plane must minimize exposure and prevent accidental disclosure.

## Decision
Store secrets encrypted at rest, separate key material from ciphertext, enforce least-privilege access, mask values in UI, prohibit logging, and audit access/change without recording values.

## Alternatives
Plain environment files; Git-encrypted files without a defined key lifecycle; returning secrets to clients on demand.

## Consequences
A key-management, rotation, backup, and recovery design is required before production. Project/server scoping must be enforced by authorization.

## Security implications
Compromise of one application or operator session must not automatically expose every secret. Emergency recovery must preserve confidentiality and auditability.
