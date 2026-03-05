// Tauri IPC utilities
import { invoke } from '@tauri-apps/api/core';
export { invoke };

/**
 * Type-safe command wrapper with error handling.
 */
export async function cmd<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args);
}

/**
 * Safe invoke — returns null on error instead of throwing.
 */
export async function safeInvoke<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T | null> {
  try {
    return await invoke<T>(command, args);
  } catch (e) {
    console.error(`[Tauri] ${command} failed:`, e);
    return null;
  }
}

/**
 * Check if running in Tauri context.
 */
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}
