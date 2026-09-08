# Secrets

**Status: Accepted MVP key-management contract**

## Scope
Secrets include GitHub, OpenRouter, Telegram, exchange, database, infrastructure, and project-specific credentials. Secret values are never stored in Git, ordinary configuration, URLs, logs, audit records, or frontend storage.

## Storage
Secret records contain ciphertext, algorithm/version metadata, scope, creation/update timestamps, and rotation state. Authenticated encryption is mandatory. Envelope encryption is preferred: secret data is encrypted with a per-record data key, while the data key is protected by the HomeHub master key.

## Master key bootstrap
On initial installation, a 256-bit random master key is generated once by the installation/bootstrap procedure on the managed server using the OS cryptographic random source. The application does not generate a replacement key on restart.

The operational copy is stored as an encrypted systemd service credential, not in SQLite, Git, or ordinary configuration. Ubuntu 24.04's systemd 255 provides `systemd-creds` and `LoadCredentialEncrypted=`; systemd can protect the credential with a host key and, where available, TPM2, then decrypt it only for the consuming service. The HomeHub service consumes the plaintext credential through its private credential directory at runtime. citeturn4search0turn4search1

The service credential is configured with least-privilege service identity and is not readable by the API's users, project processes, or other service accounts. The credential file itself is encrypted/authenticated at rest.

## Recovery escrow
A recovery copy of the master key is exported only during initial bootstrap or controlled key rotation into an offline recovery package protected by an independent recovery secret. The recovery package is not stored on the same server, not committed to Git, and not included in ordinary backups. The recovery secret is held outside the HomeHub server trust boundary (for example an enterprise password manager or hardware-backed/offline administrative store).

This separation is mandatory because an encrypted database backup without a recoverable key is not a usable backup.

## Restart and unavailable key
On restart, systemd supplies the credential to the service. If the credential cannot be decrypted or is missing, HomeHub fails closed for all secret-dependent operations. Non-secret control-plane functions may remain available only if they can operate without loading the key; the service must clearly report degraded secret availability and must not fabricate empty/default secrets.

## Backup
Database backups contain secret ciphertext and metadata, never plaintext secrets or the operational master key. The corresponding recovery escrow is managed separately. Backup manifests declare whether secret ciphertext is present and whether the required key recovery reference exists.

## Rotation
Keys are versioned. Rotation creates a new master-key version and rewraps protected data keys in a controlled transaction/workflow. Old key material remains available only until all records have been successfully rewrapped and backup/recovery references have been updated. Rotation is audited.

## Key loss / migration
If the operational credential is lost but the offline recovery package is available, a new operational credential is provisioned from the recovered master key and access is audited. If both operational and recovery copies are lost, encrypted secrets are unrecoverable by design; recovery then requires restoring the application from a state where a valid recovery key exists. Server migration requires moving the encrypted database plus the separately controlled recovery material; copying ciphertext alone is insufficient.

## Access policy
Secret metadata is scoped by server/project and follows RBAC. Reading a secret value is a separate admin-only action, requires explicit authorization, is audited, and is preferably replaced by a server-side consumer operation that avoids returning the value to a client.
