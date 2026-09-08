# ADR-004: Deployment strategy

**Status:** Accepted

## Context
Blind source pulls and restarts are difficult to audit, verify, and recover.

## Decision
Use a durable deployment state machine: authorize -> preflight -> lock -> verified backup -> exact artifact -> verification -> stage -> compatibility/migration checks -> activate -> health check -> success or deterministic failure/rollback. Every deployment has an idempotency key and durable operation state. Rollback targets an explicit known-good artifact/version plus required state references.

Required backup failure blocks destructive deployment unless a pre-approved non-destructive policy applies.

## Alternatives
`git pull` plus restart; mutable in-place deployment without backup; manual SSH deployment.

## Consequences
More state, metadata, and tests are required, but deployment becomes observable and recoverable.

## Security implications
Only immutable verified artifacts are eligible for production activation. No shell passthrough. Failure states must never be represented as success or hidden by automatic destructive retries.
