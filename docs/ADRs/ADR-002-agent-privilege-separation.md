# ADR-002: Agent privilege separation

**Status:** Proposed

## Context
Host operations sometimes require privileges, while the public Control API should remain unprivileged.

## Decision
Separate privileged execution into a server-side Agent exposing an allowlisted operation catalog over a secure authenticated local channel where topology permits.

## Alternatives
Run API as root; direct SSH; generic privileged helper with arbitrary command arguments.

## Consequences
The Agent must be secured, registered, updated, monitored, and tested. In return, API compromise does not automatically provide an unrestricted host shell.

## Security implications
The Agent is a high-trust boundary. Typed schemas, capability checks, canonical paths, timeouts, least privilege, and audit records are mandatory.
