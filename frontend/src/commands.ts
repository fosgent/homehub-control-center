/**
 * Typed wrappers around allowlisted Tauri (native) commands.
 *
 * Only specific, purpose-built commands exist here. There is no generic
 * `execute(command)` / shell / arbitrary subprocess capability anywhere in the
 * client. The UI is an unprivileged view; native I/O (version, health, updater)
 * is performed in the Rust layer, which is the authoritative allowlist.
 */

import { invoke } from "@tauri-apps/api/core";

export interface AppVersion {
  version: string;
  identifier: string;
  name: string;
}

export type ApiHealth = {
  state: "ok" | "degraded" | "unreachable" | "error";
  connected: boolean;
  status: string;
  database: string;
  version: string;
  apiUrl: string;
  detail: string;
};

export type UpdateCheckResult = {
  state: "not_configured" | "check_failed" | "up_to_date" | "update_available";
  message: string;
};

export function getAppVersion(): Promise<AppVersion> {
  return invoke<AppVersion>("get_app_version");
}

export function checkApiHealth(): Promise<ApiHealth> {
  return invoke<ApiHealth>("check_api_health");
}

export function checkForUpdates(): Promise<UpdateCheckResult> {
  return invoke<UpdateCheckResult>("check_for_updates");
}

export function getApiUrl(): Promise<string> {
  return invoke<string>("get_api_url");
}
