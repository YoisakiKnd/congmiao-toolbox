<script lang="ts">
  import { appState } from '../state.svelte';
  import { runTool } from '../tools';

  const fileActions = [
    { id: 'batch-rename' as const, icon: 'edit_square', title: 'Batch Rename', caption: 'File organization' },
    { id: 'sort-rule' as const, icon: 'sort', title: 'Sort by Rule', caption: 'Rule-based sorting' },
    { id: 'duplicate-scan' as const, icon: 'content_copy', title: 'Duplicate Scan', caption: 'Find identical files' },
  ];
</script>

<div class="tile">
  <div class="header">
    <div class="title-group">
      <div class="icon blue">
        <span class="material-symbols-rounded">folder_open</span>
      </div>
      <div>
        <p class="eyebrow">Local Files</p>
        <h3>File Management</h3>
      </div>
    </div>
  </div>

  <div class="actions">
    {#each fileActions as action}
      <button class="action-btn {appState.activeToolPulse === action.id ? 'pulse' : ''}" onclick={() => runTool(action.id)}>
        <div class="icon-wrap blue">
          <span class="material-symbols-rounded">{action.icon}</span>
        </div>
        <div class="text">
          <strong>{action.title}</strong>
          <small>{action.caption}</small>
        </div>
      </button>
    {/each}
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
    height: 100%;
  }

  .header {
    display: flex;
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

  .icon.blue {
    color: var(--accent-blue);
    background-color: var(--accent-blue-soft);
    border-color: rgba(10, 132, 255, 0.2);
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

  .actions {
    display: flex;
    flex-direction: column;
    gap: 12px;
    flex: 1;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px;
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    text-align: left;
  }

  .action-btn:hover {
    background-color: var(--bg-panel-hover);
    border-color: var(--border-focus);
    transform: translateX(4px);
  }
  
  .action-btn:active {
    transform: scale(0.98);
  }

  .action-btn.pulse {
    animation: btn-pulse 0.8s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .icon-wrap {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    border-radius: 12px;
  }

  .icon-wrap.blue {
    background-color: var(--accent-blue-soft);
    color: var(--accent-blue);
  }

  .text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .text strong {
    font-size: 15px;
    color: var(--text-primary);
    font-weight: 600;
  }

  .text small {
    font-size: 13px;
    color: var(--text-caption);
  }

  @keyframes btn-pulse {
    0% { transform: scale(1); background-color: var(--bg-panel1); }
    50% { transform: scale(1.02); background-color: var(--accent-blue-soft); border-color: var(--accent-blue); }
    100% { transform: scale(1); background-color: var(--bg-app); border-color: var(--border-subtle); }
  }
</style>
