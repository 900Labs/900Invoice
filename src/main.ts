import { mount } from 'svelte';

let renderedApp = false;

function getErrorMessage(error: unknown) {
  if (error instanceof Error) {
    return error.stack || error.message;
  }
  if (typeof error === 'string') {
    return error;
  }
  try {
    return JSON.stringify(error);
  } catch {
    return String(error);
  }
}

function renderBootError(error: unknown) {
  const target = document.getElementById('app') ?? document.body;
  const message = getErrorMessage(error) || 'Unknown startup error';

  const container = document.createElement('main');
  container.style.cssText = [
    'min-height:100vh',
    'display:flex',
    'flex-direction:column',
    'align-items:center',
    'justify-content:center',
    'gap:12px',
    'padding:32px',
    'font-family:Inter,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif',
    'color:#0f172a',
    'background:#f8fafc',
    'text-align:center',
  ].join(';');

  const title = document.createElement('h1');
  title.textContent = '900Invoice could not start';
  title.style.cssText = 'margin:0;font-size:24px;font-weight:700;color:#b91c1c';

  const details = document.createElement('pre');
  details.textContent = message;
  details.style.cssText = [
    'max-width:760px',
    'max-height:45vh',
    'overflow:auto',
    'white-space:pre-wrap',
    'word-break:break-word',
    'margin:0',
    'padding:16px',
    'border:1px solid #cbd5e1',
    'border-radius:8px',
    'background:#ffffff',
    'color:#334155',
    'font-size:13px',
    'line-height:1.5',
    'text-align:left',
  ].join(';');

  container.append(title, details);
  target.replaceChildren(container);
}

window.addEventListener('error', (event) => {
  if (!renderedApp) {
    renderBootError(event.error ?? event.message);
  }
});

window.addEventListener('unhandledrejection', (event) => {
  if (!renderedApp) {
    renderBootError(event.reason);
  }
});

async function startApp() {
  const target = document.getElementById('app');
  if (!target) {
    throw new Error('Missing #app mount target');
  }

  const { default: App } = await import('./App.svelte');
  target.replaceChildren();
  mount(App, { target });
  renderedApp = true;
}

void startApp().catch((error) => {
  renderBootError(error);
  throw error;
});
