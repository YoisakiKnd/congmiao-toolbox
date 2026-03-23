<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  type SystemStats = {
    cpu: number;
    memory_used: number;
    memory_total: number;
  };

  let stats = $state<SystemStats>({
    cpu: 0,
    memory_used: 0,
    memory_total: 0,
  });

  let timer: number;
  let loading = $state(true);

  async function fetchStats() {
    try {
      const data = await invoke<SystemStats>('get_system_stats');
      stats = data;
      loading = false;
    } catch (e) {
      console.error('Failed to fetch system stats:', e);
    }
  }

  onMount(() => {
    fetchStats();
    timer = window.setInterval(fetchStats, 1000);
  });

  onDestroy(() => {
    if (timer) clearInterval(timer);
  });

  const memoryPercent = $derived(
    stats.memory_total > 0 ? (stats.memory_used / stats.memory_total) * 100 : 0
  );
</script>

<div class="small-tile system-monitor">
  <div class="header">
    <div class="icon purple">
      <span class="material-symbols-rounded">memory</span>
    </div>
    <span class="title">System Monitor</span>
  </div>

  <div class="metrics">
    <!-- CPU Metric -->
    <div class="metric-row">
      <span class="label">CPU</span>
      <div class="value-bar">
        <div class="value">{loading ? '--' : stats.cpu.toFixed(0)}%</div>
        <div class="progress-track">
          <div class="progress-fill" style="width: {stats.cpu}%"></div>
        </div>
      </div>
    </div>

    <!-- RAM Metric -->
    <div class="metric-row">
      <span class="label">RAM</span>
      <div class="value-bar">
        <div class="value">
          {loading ? '--' : (stats.memory_used / 1024).toFixed(1)}<span class="unit">GB</span>
        </div>
        <div class="progress-track">
          <div class="progress-fill" style="width: {memoryPercent}%"></div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .small-tile {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background-color: var(--bg-panel0);
    border-radius: var(--radius-md);
    padding: 16px;
    border: 1px solid var(--border-subtle);
    box-shadow: var(--shadow-sm);
    width: 220px;
    height: 140px;
    position: relative;
    overflow: hidden;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .icon {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    border-radius: 8px;
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
  }

  .icon.purple {
    color: #AF52DE;
    background-color: rgba(175, 82, 222, 0.15);
    border-color: rgba(175, 82, 222, 0.2);
  }

  .icon .material-symbols-rounded {
    font-size: 16px;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .metrics {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .metric-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .label {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-secondary);
    width: 32px;
  }

  .value-bar {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    align-items: flex-end;
  }

  .value {
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 12px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .unit {
    font-size: 10px;
    color: var(--text-caption);
    margin-left: 2px;
  }

  .progress-track {
    width: 100%;
    height: 4px;
    background-color: var(--bg-panel1);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #AF52DE, #5E5CE6);
    border-radius: var(--radius-full);
    transition: width 0.5s cubic-bezier(0.2, 0.8, 0.2, 1);
  }
</style>
