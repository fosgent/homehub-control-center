# ADR-004: Deployment strategy

**Status:** Proposed

## Context
Blind source pulls and restarts are difficult to audit and recover.

## Decision
Use a deployment state machine: resolve release, preflight, backup, lock, stage, validate, build/install, migrate when required, activate, health-check, then success or rollback.

## Alternatives
`git pull` plus restart; mutable in-place deployment without backup; manual SSH deployment.

## Consequences
More state and storage are required, but deployments become observable, repeatable, and recoverable.

## Security implications
Prefer immutable verified artifacts. Deployment input is typed and registry-scoped. No shell passthrough is permitted.
