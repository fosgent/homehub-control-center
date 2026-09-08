# Logging

**Status: Planned**

## Categories
`Application`, `Deployment`, `System`, `Docker`, `Audit`, `Security`, `Scheduler`.

## Structured fields
Timestamp, severity, source/component, server ID, project ID when applicable, correlation/request ID, event type, operation ID, outcome, and sanitized metadata.

## Rules
Logs are structured and searchable. Secrets, access tokens, passwords, private keys, session cookies, and equivalent sensitive values must never be emitted. Error handling must sanitize upstream output before persistence or display.

## Retention
Retention is configurable by category and risk. Audit records require stronger retention/integrity controls than ordinary application logs. Cleanup is scheduled and itself audited.

## Access
Log access is authorization-protected. Sensitive operational details are not exposed before authentication. Live tailing may be added later without changing the structured event model.
