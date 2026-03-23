<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';

  let isRunning = $state(false);
  let isPrivacyEnabled = $state(false);
  let isToggling = $state(false);
  let statusTimer: ReturnType<typeof setInterval>;
  let localUrl = $state('');

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
        localUrl = '';
      } else {
        localUrl = await invoke<string>('start_peek_server');
        isRunning = true;
      }
    } catch (e) {
      console.error(e);
    }
    isToggling = false;
  };

  const togglePrivacy = async () => {
    isPrivacyEnabled = await invoke<boolean>('toggle_privacy');
  };
</script>

<div class="bento-card">
  <div class="card-header">
    <div class="title-row">
      <div class="icon-wrap blue">
        <span class="material-symbols-rounded">cast</span>
      </div>
      <div class="text-col">
        <h4>PC远程监控</h4>
        <span class="status" class:on={isRunning}>
          <span class="dot"></span> {isRunning ? '运行中 (局域网)' : '已离线'}
        </span>
      </div>
    </div>
  </div>

  <div class="card-body">
    <button class="server-btn" class:on={isRunning} disabled={isToggling} onclick={toggleServer}>
      <span class="material-symbols-rounded">{isRunning ? 'power_settings_new' : 'play_arrow'}</span>
      <span>{isRunning ? '停止服务' : '启动服务'}</span>
    </button>
    
    <div class="privacy-row">
      <span class="priv-label">隐私模糊</span>
      <button class="switch-btn" class:active={isPrivacyEnabled} onclick={togglePrivacy}>
        <div class="knob"></div>
      </button>
    </div>
  </div>
</div>

<style>
  .bento-card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: 20px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    box-shadow: var(--shadow-sm);
    transition: transform 0.2s, box-shadow 0.2s;
    width: 220px;
    height: 140px;
    position: relative;
    overflow: hidden;
  }

  .bento-card:hover {
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .icon-wrap {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 10px;
  }

  .icon-wrap.blue {
    background-color: rgba(10, 132, 255, 0.15);
    color: #0A84FF;
  }

  .icon-wrap .material-symbols-rounded {
    font-size: 18px;
  }

  .text-col {
    display: flex;
    flex-direction: column;
  }

  h4 {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .status {
    font-size: 11px;
    color: var(--text-caption);
    display: flex;
    align-items: center;
    gap: 4px;
    margin-top: 2px;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: var(--text-caption);
  }

  .status.on .dot {
    background-color: #34C759;
    box-shadow: 0 0 4px rgba(52, 199, 89, 0.8);
  }

  .status.on {
    color: #34C759;
  }

  .card-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .server-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    width: 100%;
    padding: 8px 0;
    background-color: #0A84FF;
    color: white;
    border: none;
    border-radius: 10px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .server-btn:hover {
    background-color: #0070df;
  }

  .server-btn.on {
    background-color: rgba(255, 59, 48, 0.15);
    color: #FF3B30;
  }

  .server-btn.on:hover {
    background-color: rgba(255, 59, 48, 0.25);
  }

  .server-btn .material-symbols-rounded {
    font-size: 16px;
  }

  .privacy-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 4px;
  }

  .priv-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .switch-btn {
    position: relative;
    width: 36px;
    height: 20px;
    background-color: var(--border-focus);
    border-radius: 10px;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s;
    padding: 0;
  }

  .switch-btn.active {
    background-color: #34C759;
  }

  .knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    background-color: white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .switch-btn.active .knob {
    transform: translateX(16px);
  }
</style>
