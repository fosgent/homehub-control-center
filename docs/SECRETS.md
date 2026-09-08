# Secrets

**Status: Proposed**

## Scope
Secrets may include GitHub, OpenRouter, Telegram, exchange, database, infrastructure, and project-specific credentials.

## Requirements
- Never store secrets in Git.
- Never store plaintext secrets in ordinary configuration.
- Encrypt secrets at rest using authenticated encryption.
- Separate encryption keys from encrypted secret material.
- Mask values in UI and operational responses.
- Never log secrets or full credential-bearing configuration.
- Authorize every secret access and change.
- Audit reads, writes, rotation, and deletion without recording the secret value.

## Key management
The production key hierarchy, key source, rotation, recovery, and backup policy are not yet accepted. They require a dedicated security design/ADR and should avoid a single recoverable plaintext master key on the same trust boundary as ciphertext.

## Isolation
Project secrets must be scoped to project/server and least-privilege consumers. A client should receive a secret only when a documented use case requires it; preferably the control plane or Agent performs the operation without returning the secret.

## Rotation and recovery
Rotation must support versioned secret records and controlled cutover. Recovery must preserve confidentiality and auditability and must not require committing emergency plaintext credentials.
