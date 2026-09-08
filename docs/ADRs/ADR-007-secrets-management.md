# ADR-007: Secrets management

**Status:** Accepted for MVP

## Context
Managed projects require credentials, but the control plane must minimize exposure and support usable encrypted backups.

## Decision
Store secret ciphertext using authenticated encryption with non-secret metadata. Prefer envelope encryption with per-record data keys protected by a 256-bit HomeHub master key. The operational master key is provisioned as an encrypted systemd service credential using Ubuntu 24.04/systemd 255 `LoadCredentialEncrypted=`/`systemd-creds`, with host and TPM2 protection where available. The application consumes the key only through the service credential directory.

A separately protected offline recovery package contains the master key for disaster recovery. It is not stored on the HomeHub server, in Git, or in ordinary backups. Database backups contain ciphertext and metadata only.

Rotation is versioned and audited. Key loss without the recovery package intentionally makes ciphertext unrecoverable; this is preferable to silently weakening encryption.

## Alternatives
Plain environment files; database-held master key; plaintext key beside ciphertext; Git-encrypted files without a recovery lifecycle; external KMS as an MVP dependency.

## Consequences
Bootstrap, recovery, key rotation, backup procedures, and operator custody become explicit operational requirements. External KMS integration remains a future migration path.

## Security implications
The master key is never in SQLite, Git, ordinary configuration, logs, or client storage. Secret reads are separately authorized and audited.
