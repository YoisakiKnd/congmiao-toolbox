<script lang="ts">
  import { appState } from '../state.svelte';
  import { runTool, inspectClipboardJson } from '../tools';
  import { onMount } from 'svelte';

  onMount(() => {
    const timer = setInterval(() => {
      inspectClipboardJson();
    }, 1400);
    inspectClipboardJson();
    return () => clearInterval(timer);
  });
</script>

<div class="tile {appState.jsonFlash ? 'flash' : ''} {appState.activeToolPulse === 'json-format' ? 'pulse' : ''}">
  <div class="header">
    <div class="title-group">
      <div class="icon teal">
        <span class="material-symbols-rounded">data_object</span>
      </div>
      <div>
        <p class="eyebrow">Clipboard Watch</p>
        <h3>JSON Format</h3>
      </div>
    </div>
  </div>

  <div class="status-box {appState.clipboardJsonReady ? 'ready' : ''}">
    <div class="status-header">
      <div class="status-indicator"></div>
      <strong>{appState.clipboardJsonHint}</strong>
    </div>
    <div class="preview">{appState.clipboardJsonPreview}</div>
  </div>

  <button class="action-btn" onclick={() => runTool('json-format')}>
    <div class="text text-left">
      <strong>Format & Copy JSON</strong>
      <small>Read from clipboard</small>
    </div>
    <div class="icon-wrap teal">
      <span class="material-symbols-rounded">content_copy</span>
    </div>
  </button>
</div>

<style>
  .tile {
    display: flex;
    flex-direction: column;
    gap: 16px;
    background-color: var(--bg-panel0);
    border-radius: var(--radius-lg);
    padding: 24px;
    border: 1px solid var(--border-subtle);
    box-shadow: var(--shadow-sm);
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
    height: 100%;
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

  .status-box {
    display: flex;
    flex-direction: column;
    gap: 12px;
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 16px;
    flex: 1;
    transition: all var(--transition-normal);
  }

  .status-box.ready {
    border-color: rgba(48, 209, 88, 0.3);
    background-color: rgba(48, 209, 88, 0.03);
  }

  .status-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--text-caption);
    transition: all var(--transition-normal);
  }

  .status-box.ready .status-indicator {
    background-color: var(--accent-teal);
    box-shadow: 0 0 8px var(--accent-teal);
  }

  .status-header strong {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .preview {
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 13px;
    color: var(--text-caption);
    background-color: var(--bg-panel1);
    padding: 12px;
    border-radius: var(--radius-sm);
    min-height: 44px;
    display: flex;
    align-items: center;
  }

  .status-box.ready .preview {
    color: var(--accent-teal);
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    background-color: var(--bg-panel1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    width: 100%;
  }

  .action-btn:hover {
    background-color: var(--bg-panel-hover);
    border-color: var(--border-focus);
    transform: translateY(-2px);
  }
  
  .action-btn:active {
    transform: scale(0.98);
  }

  .text-left {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 2px;
  }

  .text strong {
    font-size: 15px;
    color: var(--text-primary);
    font-weight: 600;
  }

  .text small {
    font-size: 12px;
    color: var(--text-caption);
  }

  .icon-wrap {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    border-radius: 12px;
  }

  .icon-wrap.teal {
    background-color: var(--accent-teal-soft);
    color: var(--accent-teal);
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
