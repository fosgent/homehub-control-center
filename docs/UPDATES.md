# Updates

**Status: Planned**

## Control Center self-update
```text
Check GitHub Releases -> compare versions -> obtain immutable artifact -> verify -> stage -> install -> health check -> recover/rollback if supported
```

Self-update must not blindly execute repository code. Release identity, version compatibility, integrity/signature policy, and recovery behavior must be defined before production use.

## Project update
```text
Control Center -> Deployment Engine -> backup -> acquire release -> validate -> deploy -> health check -> rollback on failure
```

Project updates must be represented as deployment records and produce audit events.

## Versioning
Use explicit semantic/application versions where appropriate. Store both desired and installed versions. Compare normalized versions using a single library/rule set rather than ad-hoc string comparison.

## Integrity
Prefer signed or otherwise cryptographically verifiable release artifacts. Trust roots and verification failures must fail closed.
