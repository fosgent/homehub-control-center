import { useCallback, useEffect, useState } from "react";
import "./App.css";

import {
  checkApiHealth,
  checkForUpdates,
  getApiUrl,
  getAppVersion,
} from "./commands";
import { StatusDot } from "./components/StatusDot";
import type { AppVersion, ApiHealth, UpdateCheckResult } from "./commands";

type Section = "settings" | "about";

const CONNECTION_TEXT: Record<ApiHealth["state"], string> = {
  ok: "Connected",
  degraded: "Connected",
  unreachable: "Disconnected",
  error: "Disconnected",
};

const CONNECTION_COLOR: Record<ApiHealth["state"], "green" | "red" | "amber"> = {
  ok: "green",
  degraded: "amber",
  unreachable: "red",
  error: "red",
};

const HEALTH_TEXT: Record<ApiHealth["state"], string> = {
  ok: "OK",
  degraded: "Degraded",
  unreachable: "Failed",
  error: "Failed",
};

const HEALTH_COLOR: Record<ApiHealth["state"], "green" | "red" | "amber"> = {
  ok: "green",
  degraded: "amber",
  unreachable: "red",
  error: "red",
};

export default function App() {
  const [version, setVersion] = useState<AppVersion | null>(null);
  const [apiUrl, setApiUrl] = useState<string>("");
  const [health, setHealth] = useState<ApiHealth | null>(null);
  const [checkingHealth, setCheckingHealth] = useState(false);
  const [updateCheck, setUpdateCheck] = useState<UpdateCheckResult | null>(null);
  const [checkingUpdates, setCheckingUpdates] = useState(false);
  const [section, setSection] = useState<Section>("settings");

  const refreshHealth = useCallback(async () => {
    setCheckingHealth(true);
    setHealth(null);
    try {
      setHealth(await checkApiHealth());
    } catch {
      setHealth({
        state: "error",
        connected: false,
        status: "unknown",
        database: "unknown",
        version: "unknown",
        apiUrl,
        detail: "Failed to reach the native layer",
      });
    } finally {
      setCheckingHealth(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    getAppVersion()
      .then(setVersion)
      .catch(() => setVersion(null));
    getApiUrl()
      .then(setApiUrl)
      .catch(() => setApiUrl("unknown"));
    refreshHealth();
  }, [refreshHealth]);

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

  const connectionState: ApiHealth["state"] = health?.state ?? "error";
  const isChecking = health === null && checkingHealth;

  const connectColor = isChecking
    ? "amber"
    : CONNECTION_COLOR[connectionState];
  const healthColor = isChecking ? "amber" : HEALTH_COLOR[connectionState];

  return (
    <div className="app">
      <header className="header">
        <h1 className="title">HomeHub Control Center</h1>
        <div className="version">Version: {version?.version ?? "…"}</div>
      </header>

      <section className="card">
        <h2 className="card-title">Control API</h2>
        <div className="status-line">
          <StatusDot color={connectColor}>
            {isChecking
              ? "Checking…"
              : CONNECTION_TEXT[connectionState]}
          </StatusDot>
          <span className="endpoint">{apiUrl || "…"}</span>
        </div>

        <h2 className="card-title">Health</h2>
        <div className="status-line">
          <StatusDot color={healthColor}>
            {isChecking ? "Checking…" : HEALTH_TEXT[connectionState]}
          </StatusDot>
        </div>
        {health && health.status !== "unknown" ? (
          <div className="detail">
            status={health.status} · database={health.database} · api=v
            {health.version}
          </div>
        ) : null}
        {health && health.detail ? (
          <div className="detail">{health.detail}</div>
        ) : null}

        <div className="actions">
          <button onClick={refreshHealth} disabled={checkingHealth}>
            {checkingHealth ? "Checking…" : "Check health"}
          </button>
          <button onClick={handleCheckForUpdates} disabled={checkingUpdates}>
            {checkingUpdates ? "Checking…" : "Check for updates"}
          </button>
        </div>
        {updateCheck ? <div className="detail">{updateCheck.message}</div> : null}
      </section>

      <nav className="nav">
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

      <section className="card">
        {section === "settings" ? (
          <div>
            <h2 className="card-title">Settings</h2>
            <p className="muted">
              Control API endpoint: <code>{apiUrl || "…"}</code>
            </p>
            <p className="muted">
              Connectivity is configured centrally; no credentials are stored
              here.
            </p>
          </div>
        ) : (
          <div>
            <h2 className="card-title">About</h2>
            <p className="muted">HomeHub Control Center</p>
            <p className="muted">
              Version <code>{version?.version ?? "…"}</code>
            </p>
            <p className="muted">
              Identifier <code>{version?.identifier ?? "…"}</code>
            </p>
          </div>
        )}
      </section>
    </div>
  );
}
