# Scheduler

**Status: Planned**

The scheduler executes durable, registered jobs rather than arbitrary commands.

Initial job classes:
- update checks
- project/server health checks
- project backups
- database backups
- log cleanup
- certificate checks

Each job records: ID, enabled state, schedule, last run, next run, status, duration, result, error, owner/scope, and correlation ID.

## Execution
Use a durable job store, per-job locks, bounded execution time, retries with backoff where safe, and idempotency rules. `Run now` must pass the same authorization and operation validation as scheduled execution.

## Reliability
A scheduler restart must recover pending work without duplicate destructive execution. Jobs must expose history and failure state. Scheduler health itself is monitored.

## Security
Schedules reference registered operations, never shell strings. Job configuration is authorization-protected and audited.
