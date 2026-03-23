<script lang="ts">
  import { appState } from '../state.svelte';
</script>

<div class="tile">
  <div class="header">
    <div class="title-group">
      <div class="icon teal">
        <span class="material-symbols-rounded">history</span>
      </div>
      <div>
        <p class="eyebrow">Event Log</p>
        <h3>Recent Activity</h3>
      </div>
    </div>
  </div>

  <div class="activity-list">
    {#each appState.recentActivity as entry, i}
      <div class="activity-item {entry.accent === 'teal' ? 'teal-border' : 'blue-border'} glow-in" style="animation-delay: {i * 0.05}s">
        <div class="item-header">
          <span class="source {entry.source.toLowerCase()}">{entry.source}</span>
          <span class="meta">{entry.meta}</span>
        </div>
        <strong>{entry.title}</strong>
        <span class="value">{entry.value}</span>
      </div>
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

  .activity-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    flex: 1;
    overflow-y: auto;
    padding-right: 4px;
  }

  .activity-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 14px 16px;
    border-radius: var(--radius-md);
    background-color: var(--bg-app);
    border: 1px solid var(--border-focus);
    border-left-width: 3px;
    transition: all var(--transition-normal);
  }

  .activity-item:hover {
    background-color: var(--bg-panel1);
  }

  .teal-border { border-left-color: var(--accent-teal); }
  .blue-border { border-left-color: var(--accent-blue); }

  .item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .source {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid var(--border-subtle);
    color: var(--text-caption);
  }

  .source.system { background-color: rgba(255, 255, 255, 0.05); }
  .source.text { background-color: var(--accent-teal-soft); color: var(--accent-teal); border-color: rgba(48, 209, 88, 0.2); }
  .source.file { background-color: var(--accent-blue-soft); color: var(--accent-blue); border-color: rgba(10, 132, 255, 0.2); }

  .meta {
    font-size: 11px;
    color: var(--text-caption);
    font-variant-numeric: tabular-nums;
  }

  strong {
    font-size: 14px;
    color: var(--text-primary);
    font-weight: 600;
  }

  .value {
    font-size: 13px;
    color: var(--text-caption);
    font-family: ui-monospace, SFMono-Regular, monospace;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .glow-in {
    animation: glowIn 0.4s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }

  @keyframes glowIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
