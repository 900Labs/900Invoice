// Navigation store using Svelte 5 runes
let currentView = $state('dashboard');
let viewParams = $state<Record<string, string>>({});
let history = $state<Array<{ view: string; params: Record<string, string> }>>([]);

export function navigateTo(view: string, params: Record<string, string> = {}) {
  history = [...history, { view: currentView, params: { ...viewParams } }];
  currentView = view;
  viewParams = params;
}

export function navigateBack() {
  if (history.length > 0) {
    const prev = history[history.length - 1];
    history = history.slice(0, -1);
    currentView = prev.view;
    viewParams = prev.params;
  } else {
    currentView = 'dashboard';
    viewParams = {};
  }
}

export function getCurrentView() {
  return currentView;
}

export function getViewParams() {
  return viewParams;
}

export function getHistory() {
  return history;
}

export function canGoBack() {
  return history.length > 0;
}
