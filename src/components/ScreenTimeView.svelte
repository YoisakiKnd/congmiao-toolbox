<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  
  interface AppUsage {
    app_name: string;
    seconds: number;
  }

  let usageData: AppUsage[] = $state([]);
  let refreshTimer: ReturnType<typeof setInterval>;
  let isLoading = $state(true);

  const fetchData = async () => {
    try {
      usageData = await invoke<AppUsage[]>('get_app_usage');
    } catch (e) {
      console.error(e);
    } finally {
      isLoading = false;
    }
  };

  onMount(async () => {
    await fetchData();
    refreshTimer = setInterval(fetchData, 5000);
  });

  onDestroy(() => {
    if (refreshTimer) clearInterval(refreshTimer);
  });

  let totalSeconds = $derived(usageData.reduce((acc, obj) => acc + obj.seconds, 0));
  let maxSeconds = $derived(usageData.length > 0 ? usageData[0].seconds : 1);

  const formatTime = (secs: number) => {
    if (secs < 60) return `${secs} 秒`;
    const mins = Math.floor(secs / 60);
    const h = Math.floor(mins / 60);
    const m = mins % 60;
    if (h > 0) return `${h} 小时 ${m} 分钟`;
    return `${m} 分钟`;
  };
</script>

<div class="screen-time-view">
  <div class="header-section">
    <div class="title-row">
      <div class="icon-wrap indigo">
        <span class="material-symbols-rounded">schedule</span>
      </div>
      <div class="text-group">
        <h3>应用使用时长</h3>
        <p>自动记录你在各个软件上倾注的时间（数据仅保存在本地）</p>
      </div>
    </div>
  </div>

  <div class="summary-card">
    <span class="summary-label">累计屏幕时间</span>
    <h2 class="total-time">{formatTime(totalSeconds)}</h2>
  </div>

  <div class="usage-list">
    {#each usageData as item (item.app_name)}
      <div class="usage-item">
        <div class="app-info">
          <span class="app-name" title={item.app_name}>{item.app_name}</span>
          <span class="app-time">{formatTime(item.seconds)}</span>
        </div>
        <div class="progress-track">
          <div class="progress-fill" style="width: {(item.seconds / maxSeconds) * 100}%"></div>
        </div>
      </div>
    {/each}
    {#if usageData.length === 0 && !isLoading}
      <div class="empty-state">
        <span class="material-symbols-rounded">hourglass_empty</span>
        <p>暂无使用数据，正在持续收集...</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .screen-time-view {
    display: flex;
    flex-direction: column;
    gap: 32px;
    height: 100%;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
    padding-top: 16px;
  }

  .header-section {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .icon-wrap {
    display: grid;
    place-items: center;
    width: 48px;
    height: 48px;
    border-radius: 14px;
    border: 1px solid var(--border-subtle);
  }

  .icon-wrap.indigo {
    color: #5E5CE6;
    background-color: rgba(94, 92, 230, 0.15);
    border-color: rgba(94, 92, 230, 0.2);
  }

  .icon-wrap .material-symbols-rounded {
    font-size: 24px;
  }

  .text-group h3 {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
    margin-bottom: 4px;
  }

  .text-group p {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .summary-card {
    background: linear-gradient(135deg, rgba(94, 92, 230, 0.1), rgba(94, 92, 230, 0.02));
    border: 1px solid rgba(94, 92, 230, 0.2);
    border-radius: var(--radius-lg);
    padding: 32px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
  }

  .summary-label {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .total-time {
    font-size: 42px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .usage-list {
    display: flex;
    flex-direction: column;
    gap: 16px;
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 24px;
    box-shadow: var(--shadow-sm);
  }

  .usage-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .app-info {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
  }

  .app-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 60%;
  }

  .app-time {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .progress-track {
    width: 100%;
    height: 12px;
    background-color: var(--bg-panel1);
    border-radius: 6px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #5E5CE6, #af52de);
    border-radius: 6px;
    transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 40px 0;
    color: var(--text-caption);
  }

  .empty-state .material-symbols-rounded {
    font-size: 32px;
    opacity: 0.5;
  }
</style>
