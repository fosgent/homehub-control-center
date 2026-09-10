import { useState, type FormEvent } from "react";
import type { ServerConfigInput, ServerView } from "../commands";

/** Client-side URL guard mirroring the native validator. The native layer is
 *  authoritative; this only gives immediate feedback before submit. */
export function validateEndpointUrl(raw: string): string | null {
  const value = raw.trim();
  if (!value) {
    return "Endpoint URL is required.";
  }
  let parsed: URL;
  try {
    parsed = new URL(value);
  } catch {
    return "Endpoint URL is invalid.";
  }
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    return "Only http:// and https:// endpoints are supported.";
  }
  if (parsed.username || parsed.password) {
    return "Endpoint URL must not embed credentials.";
  }
  return null;
}

interface ServerFormProps {
  /** Existing server when editing; null when adding a new server. */
  initial: ServerView | null;
  busy: boolean;
  error: string | null;
  onCancel: () => void;
  onSubmit: (input: ServerConfigInput) => void;
}

export function ServerForm({
  initial,
  busy,
  error,
  onCancel,
  onSubmit,
}: ServerFormProps) {
  const [name, setName] = useState(initial?.name ?? "");
  const [url, setUrl] = useState(initial?.url ?? "http://127.0.0.1:8000");
  const [enabled, setEnabled] = useState(initial?.enabled ?? true);
  const [localError, setLocalError] = useState<string | null>(null);

  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();
    const nameTrimmed = name.trim();
    if (!nameTrimmed) {
      setLocalError("Server name is required.");
      return;
    }
    const urlError = validateEndpointUrl(url);
    if (urlError) {
      setLocalError(urlError);
      return;
    }
    setLocalError(null);
    onSubmit({
      serverId: initial?.serverId ?? null,
      name: nameTrimmed,
      url: url.trim(),
      enabled,
    });
  };

  return (
    <form className="server-form" onSubmit={handleSubmit}>
      <div className="form-row">
        <label htmlFor="server-name">Name</label>
        <input
          id="server-name"
          type="text"
          value={name}
          placeholder="HomeHub Local"
          onChange={(e) => setName(e.target.value)}
          disabled={busy}
        />
      </div>
      <div className="form-row">
        <label htmlFor="server-url">Control API URL</label>
        <input
          id="server-url"
          type="text"
          value={url}
          placeholder="http://127.0.0.1:8000"
          onChange={(e) => setUrl(e.target.value)}
          disabled={busy}
        />
      </div>
      <label className="form-check">
        <input
          type="checkbox"
          checked={enabled}
          onChange={(e) => setEnabled(e.target.checked)}
          disabled={busy}
        />
        Enabled
      </label>
      {(localError ?? error) ? (
        <div className="form-error" role="alert">
          {localError ?? error}
        </div>
      ) : null}
      <div className="form-actions">
        <button type="submit" disabled={busy}>
          {busy ? "Saving…" : initial ? "Save" : "Add Server"}
        </button>
        <button type="button" onClick={onCancel} disabled={busy}>
          Cancel
        </button>
      </div>
    </form>
  );
}