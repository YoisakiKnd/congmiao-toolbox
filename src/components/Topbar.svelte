<script lang="ts">
  import { appState } from '../state.svelte';
  import { runTool } from '../tools';

  const currentTime = $derived(() => {
    const d = new Date(appState.currentUnix * 1000);
    return d.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false });
  });

  const currentDate = $derived(() => {
    const d = new Date(appState.currentUnix * 1000);
    const months = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月'];
    const days = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'];
    return `${d.getFullYear()}年${months[d.getMonth()]}${d.getDate()}日 ${days[d.getDay()]}`;
  });
</script>

<header class="topbar">
  <div class="topbar-title">
    <p class="eyebrow">{currentDate()}</p>
    <h2>{currentTime()}</h2>
  </div>

  <div class="topbar-actions">
    <button
      class="icon-btn"
      aria-label="切换主题"
      onclick={() => appState.toggleTheme()}
    >
      <span class="material-symbols-rounded">
        {appState.theme === 'dark' ? 'light_mode' : 'dark_mode'}
      </span>
    </button>
  </div>
</header>

<style>
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 80px;
    padding: 0 32px;
    background-color: var(--bg-app);
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .topbar-title {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .eyebrow {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent-blue);
  }

  h2 {
    font-size: 24px;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }

  .topbar-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .icon-btn {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-full);
    background-color: var(--bg-panel0);
    color: var(--text-secondary);
    border: 1px solid var(--border-subtle);
    transition: all var(--transition-fast);
  }

  .icon-btn:hover {
    background-color: var(--bg-panel-hover);
    color: var(--text-primary);
    border-color: var(--border-focus);
  }
</style>
