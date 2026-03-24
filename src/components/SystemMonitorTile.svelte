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

  const cpuColor = $derived(
    stats.cpu > 80 ? '#FF3B30' : stats.cpu > 50 ? '#FF9500' : '#5E5CE6'
  );
</script>

<div class="widget-card">
  <div class="card-top">
    <div class="icon-badge purple">
      <span class="material-symbols-rounded">memory</span>
    </div>
    <span class="card-title">系统监视器</span>
  </div>

  <div class="metrics">
    <div class="metric">
      <div class="metric-header">
        <span class="metric-label">CPU</span>
        <span class="metric-value">{loading ? '--' : stats.cpu.toFixed(0)}%</span>
      </div>
      <div class="bar-track">
        <div class="bar-fill" style="width: {stats.cpu}%; background-color: {cpuColor}"></div>
      </div>
    </div>

    <div class="metric">
      <div class="metric-header">
        <span class="metric-label">RAM</span>
        <span class="metric-value">{loading ? '--' : (stats.memory_used / 1024).toFixed(1)} <span class="unit">GB</span></span>
      </div>
      <div class="bar-track">
        <div class="bar-fill purple" style="width: {memoryPercent}%"></div>
      </div>
    </div>
  </div>
</div>

<style>
  .widget-card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: 16px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: var(--shadow-sm);
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .widget-card:hover {
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }

  .card-top {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .icon-badge {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 10px;
    flex-shrink: 0;
  }

  .icon-badge.purple {
    color: #5E5CE6;
    background-color: rgba(94, 92, 230, 0.12);
  }

  .icon-badge .material-symbols-rounded { font-size: 18px; }

  .card-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .metrics {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .metric-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 5px;
  }

  .metric-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .metric-value {
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .unit {
    font-size: 10px;
    color: var(--text-caption);
  }

  .bar-track {
    width: 100%;
    height: 6px;
    background-color: var(--bg-panel1);
    border-radius: 3px;
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.6s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .bar-fill.purple {
    background: linear-gradient(90deg, #5E5CE6, #AF52DE);
  }
</style>
