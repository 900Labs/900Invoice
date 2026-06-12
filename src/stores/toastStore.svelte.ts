// Toast store using Svelte 5 runes
export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  message: string;
  type: ToastType;
  duration: number;
}

let toasts = $state<Toast[]>([]);

function generateId(): string {
  return Math.random().toString(36).slice(2, 9);
}

export function showToast(message: string, type: ToastType = 'info', duration: number = 5000) {
  const id = generateId();
  toasts = [...toasts, { id, message, type, duration }];

  if (duration > 0) {
    setTimeout(() => {
      dismissToast(id);
    }, duration);
  }
}

export function dismissToast(id: string) {
  toasts = toasts.filter(t => t.id !== id);
}

export function getToasts() {
  return toasts;
}

export function success(message: string) {
  showToast(message, 'success');
}

export function error(message: string) {
  showToast(message, 'error');
}

export function warning(message: string) {
  showToast(message, 'warning');
}

export function info(message: string) {
  showToast(message, 'info');
}
