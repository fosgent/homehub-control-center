/**
 * Typed wrappers around allowlisted Tauri (native) commands.
 *
 * Only specific, purpose-built commands exist here. There is no generic
 * `execute(command)` / shell / arbitrary subprocess capability anywhere in the
 * client. The UI is an unprivileged view; native I/O (version, server registry,
 * health probes, updater) is performed in the Rust layer, which is the
 * authoritative allowlist.
 */

import { invoke } from "@tauri-apps/api/core";

export interface AppVersion {
  version: string;
  identifier: string;
  name: string;
}

/** Runtime connection status of a configured server. */
export type ConnectionState =
  | "disconnected"
  | "connecting"
  | "connected"
  | "error"
  | "disabled";

/** Classification of a connection test / health probe. */
export type TestOutcome =
  | "healthy"
  | "degraded"
  | "unreachable"
  | "invalid_endpoint"
  | "http_error"
  | "invalid_response";

export interface ConnectionTest {
  outcome: TestOutcome;
  connected: boolean;
  degraded: boolean;
  message: string;
  serverUrl: string;
  healthUrl: string;
  status: string;
  database: string;
  version: string;
}

/** A configured server plus its runtime state. */
export interface ServerView {
  serverId: string;
  name: string;
  url: string;
  enabled: boolean;
  status: ConnectionState;
  degraded: boolean;
  error: string | null;
  lastCheck: ConnectionTest | null;
}

/** Payload for creating (serverId null) or updating a server. */
export interface ServerConfigInput {
  serverId: string | null;
  name: string;
  url: string;
  enabled: boolean;
}

export type UpdateCheckResult = {
  state: "not_configured" | "check_failed" | "up_to_date" | "update_available";
  message: string;
};

export function getAppVersion(): Promise<AppVersion> {
  return invoke<AppVersion>("get_app_version");
}

export function listServers(): Promise<ServerView[]> {
  return invoke<ServerView[]>("list_servers");
}

export function saveServer(input: ServerConfigInput): Promise<ServerView> {
  return invoke<ServerView>("save_server", { input });
}

export function removeServer(serverId: string): Promise<ServerView[]> {
  return invoke<ServerView[]>("remove_server", { serverId });
}

export function testConnection(serverId: string): Promise<ServerView> {
  return invoke<ServerView>("test_connection", { serverId });
}

export function connectServer(serverId: string): Promise<ServerView> {
  return invoke<ServerView>("connect_server", { serverId });
}

export function disconnectServer(serverId: string): Promise<ServerView> {
  return invoke<ServerView>("disconnect_server", { serverId });
}

export function checkForUpdates(): Promise<UpdateCheckResult> {
  return invoke<UpdateCheckResult>("check_for_updates");
}