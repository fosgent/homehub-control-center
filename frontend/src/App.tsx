import { useCallback, useEffect, useState } from "react";
import "./App.css";

import {
  checkForUpdates,
  connectServer,
  disconnectServer,
  getAppVersion,
  listServers,
  removeServer,
  saveServer,
  testConnection,
} from "./commands";
import type {
  AppVersion,
  ConnectionState,
  ServerConfigInput,
  ServerView,
  UpdateCheckResult,
} from "./commands";
import { ServerForm } from "./components/ServerForm";
import { StatusDot } from "./components/StatusDot";

type Section = "servers" | "settings" | "about";

const STATUS_DOT: Record<
  ConnectionState,
  "green" | "red" | "gray" | "amber"
> = {
  connected: "green",
  connecting: "amber",
  error: "red",
  disconnected: "gray",
  disabled: "gray",
};

const STATUS_TEXT: Record<ConnectionState, string> = {
  connected: "Connected",
  connecting: "Connecting…",
  error: "Error",
  disconnected: "Disconnected",
  disabled: "Disabled",
};

function errorMessage(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }
  if (error instanceof Error) {
    return error.message;
  }
  return "Unknown error.";
}

export default function App() {
  const [version, setVersion] = useState<AppVersion | null>(null);
  const [servers, setServers] = useState<ServerView[]>([]);
  const [section, setSection] = useState<Section>("servers");
  const [editing, setEditing] = useState<ServerView | "new" | null>(null);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [formBusy, setFormBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [formError, setFormError] = useState<string | null>(null);
  const [updateCheck, setUpdateCheck] = useState<UpdateCheckResult | null>(null);
  const [checkingUpdates, setCheckingUpdates] = useState(false);

  const refresh = useCallback(async () => {
    setServers(await listServers());
  }, []);

  useEffect(() => {
    getAppVersion()
      .then(setVersion)
      .catch(() => setVersion(null));
    refresh().catch(() => setServers([]));
  }, [refresh]);

  const runAction = useCallback(
    async (serverId: string, action: () => Promise<unknown>) => {
      setBusyId(serverId);
      setNotice(null);
      try {
        await action();
      } catch (error) {
        setNotice(errorMessage(error));
      } finally {
        setBusyId(null);
        refresh().catch(() => setServers([]));
      }
    },
    [refresh]
  );

  const handleConnect = useCallback(
    (server: ServerView) => {
      if (!server.enabled) {
        setNotice(`Cannot connect: \`${server.name}\` is disabled.`);
        return;
      }
      return runAction(server.serverId, () => connectServer(server.serverId));
    },
    [runAction]
  );

  const handleDisconnect = useCallback(
    (server: ServerView) =>
      runAction(server.serverId, () => disconnectServer(server.serverId)),
    [runAction]
  );

  const handleTest = useCallback(
    (server: ServerView) => {
      if (!server.enabled) {
        setNotice(`Cannot test connection: \`${server.name}\` is disabled.`);
        return;
      }
      return runAction(server.serverId, () => testConnection(server.serverId));
    },
    [runAction]
  );

  const handleRemove = useCallback(
    (server: ServerView) =>
      runAction(server.serverId, () => removeServer(server.serverId)),
    [runAction]
  );

  const handleSave = useCallback(
    async (input: ServerConfigInput) => {
      setFormBusy(true);
      setFormError(null);
      try {
        await saveServer(input);
        setEditing(null);
        setNotice(
          input.serverId
            ? `Saved \`${input.name}\`.`
            : `Added \`${input.name}\`.`
        );
        await refresh();
      } catch (error) {
        setFormError(errorMessage(error));
      } finally {
        setFormBusy(false);
      }
    },
    [refresh]
  );

  const handleCheckForUpdates = useCallback(async () => {
    setCheckingUpdates(true);
    setUpdateCheck(null);
    try {
      setUpdateCheck(await checkForUpdates());
    } catch {
      setUpdateCheck({
        state: "check_failed",
        message: "Failed to query the updater",
      });
    } finally {
      setCheckingUpdates(false);
    }
  }, []);

  return (
    <div className="app">
      <header className="header">
        <h1 className="title">HomeHub Control Center</h1>
        <div className="version">Version: {version?.version ?? "…"}</div>
      </header>

      <nav className="nav">
        <button
          className={section === "servers" ? "nav-active" : ""}
          onClick={() => setSection("servers")}
        >
          Servers
        </button>
        <button
          className={section === "settings" ? "nav-active" : ""}
          onClick={() => setSection("settings")}
        >
          Settings
        </button>
        <button
          className={section === "about" ? "nav-active" : ""}
          onClick={() => setSection("about")}
        >
          About
        </button>
      </nav>

      {notice ? (
        <div className="notice" role="alert">
          {notice}
        </div>
      ) : null}

      {section === "servers" ? (
        <section className="card">
          <h2 className="card-title">Servers</h2>
          {servers.length === 0 ? (
            <p className="muted">No servers configured yet.</p>
          ) : (
            <div className="server-list">
              {servers.map((server) => (
                <ServerRow
                  key={server.serverId}
                  server={server}
                  busy={busyId === server.serverId}
                  onConnect={() => handleConnect(server)}
                  onDisconnect={() => handleDisconnect(server)}
                  onTest={() => handleTest(server)}
                  onEdit={() => {
                    setFormError(null);
                    setEditing(server);
                  }}
                  onRemove={() => handleRemove(server)}
                />
              ))}
            </div>
          )}

          {editing === null ? (
            <div className="actions">
              <button
                onClick={() => {
                  setFormError(null);
                  setEditing("new");
                }}
              >
                Add Server
              </button>
            </div>
          ) : (
            <ServerForm
              initial={editing === "new" ? null : editing}
              busy={formBusy}
              error={formError}
              onCancel={() => setEditing(null)}
              onSubmit={handleSave}
            />
          )}
        </section>
      ) : null}

      {section === "settings" ? (
        <section className="card">
          <h2 className="card-title">Settings</h2>
          <p className="muted">
            Server endpoint configuration lives under <strong>Servers</strong>.
            Each server stores its Control API URL only — no passwords, API
            keys, or tokens are kept by the client.
          </p>
          <div className="actions">
            <button
              onClick={handleCheckForUpdates}
              disabled={checkingUpdates}
            >
              {checkingUpdates ? "Checking…" : "Check for updates"}
            </button>
          </div>
          {updateCheck ? (
            <div className="detail">{updateCheck.message}</div>
          ) : null}
        </section>
      ) : null}

      {section === "about" ? (
        <section className="card">
          <h2 className="card-title">About</h2>
          <p className="muted">HomeHub Control Center</p>
          <p className="muted">
            Version <code>{version?.version ?? "…"}</code>
          </p>
          <p className="muted">
            Identifier <code>{version?.identifier ?? "…"}</code>
          </p>
        </section>
      ) : null}
    </div>
  );
}

interface ServerRowProps {
  server: ServerView;
  busy: boolean;
  onConnect: () => void;
  onDisconnect: () => void;
  onTest: () => void;
  onEdit: () => void;
  onRemove: () => void;
}

function ServerRow({
  server,
  busy,
  onConnect,
  onDisconnect,
  onTest,
  onEdit,
  onRemove,
}: ServerRowProps) {
  const degraded = server.enabled && server.degraded;
  const dotColor: "green" | "red" | "gray" | "amber" = degraded
    ? "amber"
    : STATUS_DOT[server.status];
  const statusText = degraded
    ? "Connected (degraded)"
    : STATUS_TEXT[server.status];

  return (
    <div className="server">
      <div className="status-line">
        <StatusDot color={dotColor}>{statusText}</StatusDot>
        {server.enabled ? null : <span className="muted">(disabled)</span>}
      </div>
      <div className="server-name">{server.name}</div>
      <div className="endpoint">{server.url}</div>
      {server.error ? (
        <div className="error-line" role="alert">
          {server.error}
        </div>
      ) : null}
      {server.lastCheck ? (
        <div className="detail">
          {server.lastCheck.message}
          {server.lastCheck.version !== "unknown"
            ? ` (API v${server.lastCheck.version})`
            : ""}
        </div>
      ) : null}
      <div className="actions">
        {server.status === "connected" ? (
          <button onClick={onDisconnect} disabled={busy}>
            Disconnect
          </button>
        ) : (
          <button
            onClick={onConnect}
            disabled={busy || !server.enabled}
          >
            Connect
          </button>
        )}
        <button
          className="btn-secondary"
          onClick={onTest}
          disabled={busy || !server.enabled}
        >
          Test Connection
        </button>
        <button className="btn-secondary" onClick={onEdit} disabled={busy}>
          Edit
        </button>
        <button className="btn-secondary" onClick={onRemove} disabled={busy}>
          Remove
        </button>
      </div>
    </div>
  );
}