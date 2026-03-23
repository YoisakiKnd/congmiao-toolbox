<script lang="ts">
  import { appState } from '../state.svelte';
  import { runTimestampFromDrop, runTool } from '../tools';

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    appState.timestampDropActive = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    appState.timestampDropActive = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    appState.timestampDropActive = false;
    const file = e.dataTransfer?.files?.[0];
    if (file) {
      await runTimestampFromDrop(file);
    }
  }

  $effect(() => {
    const timer = setInterval(() => {
      appState.currentUnix = Math.floor(Date.now() / 1000);
    }, 1000);
    return () => clearInterval(timer);
  });

  const currentTimeText = $derived(() => {
    const date = new Date(appState.currentUnix * 1000);
    return date.toLocaleTimeString('zh-CN', { hour12: false }) + '.' + date.getMilliseconds().toString().padStart(3, '0').slice(0, 1);
  });
</script>

<div
  class="tile timestamp-tile {appState.timestampDropActive ? 'drop-active' : ''} {appState.timestampFlash ? 'flash' : ''} {appState.activeToolPulse === 'timestamp' ? 'pulse' : ''}"
  ondragover={handleDragOver}
  ondragenter={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  role="region"
  aria-label="Timestamp panel"
>
  <div class="header">
    <div class="title-group">
      <div class="icon teal">
        <span class="material-symbols-rounded">schedule</span>
      </div>
      <div>
        <p class="eyebrow">Live Tile</p>
        <h3>Timestamp Convert</h3>
      </div>
    </div>
    <div class="badge">Drop File</div>
  </div>

  <div class="display">
    <span class="label">Unix Now</span>
    <strong class="font-mono">{appState.currentUnix}</strong>
    <span class="meta">{currentTimeText()}</span>
  </div>

  <div class="actions">
    <button class="action-btn" onclick={() => runTool('timestamp')}>
      <div class="icon-wrap teal">
        <span class="material-symbols-rounded">content_copy</span>
      </div>
      <div class="text">
        <strong>Copy Unix</strong>
        <small>Current Time</small>
      </div>
    </button>
    <button class="action-btn" onclick={() => runTool('url-encode')}>
      <div class="icon-wrap teal">
        <span class="material-symbols-rounded">link</span>
      </div>
      <div class="text">
        <strong>URL Encode</strong>
        <small>Clipboard</small>
      </div>
    </button>
    <button class="action-btn" onclick={() => runTool('base64')}>
      <div class="icon-wrap teal">
        <span class="material-symbols-rounded">encrypted</span>
      </div>
      <div class="text">
        <strong>Base64</strong>
        <small>Clipboard</small>
      </div>
    </button>
  </div>
</div>

<style>
  .tile {
    display: flex;
    flex-direction: column;
    gap: 20px;
    background-color: var(--bg-panel0);
    border-radius: var(--radius-lg);
    padding: 24px;
    border: 1px solid var(--border-subtle);
    box-shadow: var(--shadow-sm);
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
  }

  .tile::before {
    content: '';
    position: absolute;
    inset: 0;
    pointer-events: none;
    background: linear-gradient(145deg, rgba(255,255,255,0.03) 0%, transparent 100%);
  }

  .drop-active {
    border-color: var(--accent-teal);
    background-color: rgba(48, 209, 88, 0.05);
    transform: scale(1.02);
  }

  .flash {
    animation: flash 0.6s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .pulse {
    animation: pulse 0.8s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .title-group {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .icon {
    display: grid;
    place-items: center;
    width: 48px;
    height: 48px;
    border-radius: 14px;
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
  }

  .icon.teal {
    color: var(--accent-teal);
    background-color: var(--accent-teal-soft);
    border-color: rgba(48, 209, 88, 0.2);
  }

  .eyebrow {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-caption);
    margin-bottom: 4px;
  }

  h3 {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .badge {
    font-size: 11px;
    font-weight: 700;
    padding: 6px 12px;
    border-radius: var(--radius-full);
    background-color: var(--bg-panel1);
    color: var(--text-secondary);
    border: 1px solid var(--border-subtle);
  }

  .display {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: var(--bg-app);
    border-radius: var(--radius-md);
    padding: 32px 24px;
    border: 1px solid var(--border-subtle);
    flex: 1;
  }

  .label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-caption);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 8px;
  }

  .font-mono {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 56px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.04em;
    line-height: 1;
    margin-bottom: 12px;
  }

  .meta {
    font-size: 16px;
    font-weight: 500;
    color: var(--accent-teal);
    font-family: ui-monospace, SFMono-Regular, monospace;
  }

  .actions {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .action-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 12px;
    padding: 16px 12px;
    background-color: var(--bg-panel1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
  }

  .action-btn:hover {
    background-color: var(--bg-panel-hover);
    border-color: var(--border-focus);
    transform: translateY(-2px);
  }
  
  .action-btn:active {
    transform: scale(0.96);
  }

  .icon-wrap {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: 10px;
  }

  .icon-wrap.teal {
    background-color: var(--accent-teal-soft);
    color: var(--accent-teal);
  }

  .text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .text strong {
    font-size: 14px;
    color: var(--text-primary);
    font-weight: 600;
  }

  .text small {
    font-size: 11px;
    color: var(--text-caption);
  }

  @keyframes flash {
    0% { background-color: var(--bg-panel0); border-color: var(--border-subtle); }
    50% { background-color: var(--accent-teal-soft); border-color: var(--accent-teal); }
    100% { background-color: var(--bg-panel0); border-color: var(--border-subtle); }
  }

  @keyframes pulse {
    0% { transform: scale(1); }
    50% { transform: scale(1.02); }
    100% { transform: scale(1); }
  }
</style>
