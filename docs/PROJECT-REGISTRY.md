# Project Registry

**Status: Accepted foundation model**

The registry is the canonical Control Plane inventory of managed projects. It is persisted domain data, not an executable YAML file. YAML may later be supported as import/export only after schema validation and authorization.

## Project identity
Each project has a stable `project_id`, display name, `server_id`, lifecycle status, repository/artifact source, deployment/runtime adapter types, health-check definition, capability set, desired/installed version, deployment state, and backup references.

## Managed resources
Each project declares canonical roots derived from the server's managed-root policy:

```text
app_root     -> /srv/apps/<project>
data_root    -> /srv/data/<project>
config_root  -> /srv/configs/<project>
backup_root  -> /srv/backups/<project>
```

The registry never accepts a client-selected replacement root during an operation. The Agent resolves these roots from the registry and validates them against the server's allowed managed-root set.

## Adapter selection
`deployment.type`, `runtime.type`, and capability identifiers select registered adapters. Unknown adapter types/capabilities are rejected. Registry data cannot contain shell templates, command strings, executable paths, or unbounded environment interpolation.

## Mutation rules
Registry changes require admin authorization, schema validation, optimistic concurrency/version checks, and an audit event. A registry mutation does not automatically grant a new Agent capability. Destructive removal requires explicit confirmation and a retention decision for app/data/config/backup resources.

## Filesystem safety
Relative subpaths used by typed operations are validated by the Agent. Absolute paths, traversal, symlink escapes, cross-project references, and resource-root overrides are forbidden. The registry is the source of resource identity; it is not a permission bypass.
