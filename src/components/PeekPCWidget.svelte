<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';

  let isRunning = $state(false);
  let isPrivacyEnabled = $state(false);
  let isToggling = $state(false);
  let statusTimer: ReturnType<typeof setInterval>;

  const checkStatus = async () => {
    isRunning = await invoke<boolean>('get_peek_status');
    isPrivacyEnabled = await invoke<boolean>('get_privacy_status');
  };

  onMount(async () => {
    await checkStatus();
    statusTimer = setInterval(checkStatus, 3000);
  });

  onDestroy(() => {
    if (statusTimer) clearInterval(statusTimer);
  });

  const toggleServer = async () => {
    if (isToggling) return;
    isToggling = true;
    try {
      if (isRunning) {
        await invoke('stop_peek_server');
        isRunning = false;
      } else {
        await invoke<string>('start_peek_server');
        isRunning = true;
      }
    } catch (e) { console.error(e); }
    isToggling = false;
  };

  const togglePrivacy = async () => {
    isPrivacyEnabled = await invoke<boolean>('toggle_privacy');
  };
</script>

<div class="widget-card">
  <div class="card-top">
    <div class="icon-badge blue">
      <span class="material-symbols-rounded">cast</span>
    </div>
    <div class="title-col">
      <span class="card-title">PC 远程监控</span>
      <span class="status-tag" class:on={isRunning}>
        <span class="dot"></span>
        {isRunning ? '运行中' : '已离线'}
      </span>
    </div>
  </div>

  <div class="card-body">
    <button class="action-btn" class:running={isRunning} disabled={isToggling} onclick={toggleServer}>
      <span class="material-symbols-rounded">{isRunning ? 'stop_circle' : 'play_arrow'}</span>
      {isRunning ? '停止服务' : '启动服务'}
    </button>

    <div class="toggle-row">
      <span class="toggle-label">隐私模糊</span>
      <button class="switch" class:active={isPrivacyEnabled} onclick={togglePrivacy}>
        <div class="knob"></div>
      </button>
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
    gap: 14px;
    box-shadow: var(--shadow-sm);
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .widget-card:hover {
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }

  .card-top { display: flex; align-items: center; gap: 10px; }

  .icon-badge {
    display: grid; place-items: center;
    width: 32px; height: 32px; border-radius: 10px; flex-shrink: 0;
  }

  .icon-badge.blue { color: #0A84FF; background-color: rgba(10, 132, 255, 0.12); }
  .icon-badge .material-symbols-rounded { font-size: 18px; }

  .title-col { display: flex; flex-direction: column; gap: 2px; }
  .card-title { font-size: 14px; font-weight: 600; color: var(--text-primary); }

  .status-tag {
    font-size: 11px; font-weight: 500; color: var(--text-caption);
    display: flex; align-items: center; gap: 4px;
  }
  .dot { width: 6px; height: 6px; border-radius: 50%; background-color: var(--text-caption); flex-shrink: 0; }
  .status-tag.on .dot { background-color: #34C759; box-shadow: 0 0 4px rgba(52, 199, 89, 0.6); }
  .status-tag.on { color: #34C759; }

  .card-body { display: flex; flex-direction: column; gap: 10px; }

  .action-btn {
    display: flex; align-items: center; justify-content: center; gap: 6px;
    width: 100%; padding: 8px 0; background-color: #0A84FF; color: white;
    border: none; border-radius: 10px; font-size: 12px; font-weight: 600;
    cursor: pointer; transition: all 0.2s;
  }
  .action-btn:hover { background-color: #0070df; }
  .action-btn.running { background-color: rgba(255, 59, 48, 0.12); color: #FF3B30; }
  .action-btn.running:hover { background-color: rgba(255, 59, 48, 0.22); }
  .action-btn .material-symbols-rounded { font-size: 16px; }

  .toggle-row { display: flex; justify-content: space-between; align-items: center; padding: 0 2px; }
  .toggle-label { font-size: 12px; font-weight: 600; color: var(--text-secondary); }

  .switch {
    position: relative; width: 36px; height: 20px;
    background-color: var(--bg-panel1); border-radius: 10px;
    border: 1px solid var(--border-subtle); cursor: pointer;
    transition: background-color 0.3s; padding: 0;
  }
  .switch.active { background-color: #34C759; border-color: #34C759; }

  .knob {
    position: absolute; top: 2px; left: 2px;
    width: 14px; height: 14px;
    background-color: white; border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0,0,0,0.2);
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }
  .switch.active .knob { transform: translateX(16px); }
</style>
