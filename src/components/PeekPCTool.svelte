<script lang="ts">
  import { appState } from '../state.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';

  let isRunning = $state(false);
  let localUrl = $state('');
  let isPrivacyEnabled = $state(false);
  let statusTimer: ReturnType<typeof setInterval>;
  let isToggling = $state(false);

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

  const handleToggleServer = async () => {
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

  const handleTogglePrivacy = async () => {
    isPrivacyEnabled = await invoke<boolean>('toggle_privacy');
  };

  const handleCopy = async () => {
    if (!localUrl) return;
    try { await navigator.clipboard.writeText(localUrl); } 
    catch (e) { console.error('Copy failed'); }
  };
</script>

<div class="tool-view">
  <button class="back-btn" onclick={() => appState.activeToolId = null}>
    <span class="material-symbols-rounded">arrow_back</span>
    <span>全部功能</span>
  </button>

  <div class="header">
    <div class="header-left">
      <div class="icon blue">
        <span class="material-symbols-rounded">desktop_windows</span>
      </div>
      <div>
        <p class="eyebrow">跨终端交互</p>
        <h3>Peek PC 远程监视</h3>
      </div>
    </div>
  </div>

  <div class="content-area">
    <div class="main-card">
      <div class="status-indicator" class:running={isRunning}>
        <div class="pulse-dot"></div>
        <span class="status-text">{isRunning ? '服务运行中' : '服务已停止'}</span>
      </div>

      <p class="desc">
        启动后，你可以在同一局域网下的手机或另一台电脑浏览器中，实时查看本机的硬件状态与屏幕截图。（完全脱离原有的 Go 环境，100% Rust 重构，极低内存消耗）
      </p>

      <div class="power-section">
        <button 
          class="power-btn" 
          class:on={isRunning} 
          disabled={isToggling}
          onclick={handleToggleServer}
        >
          <span class="material-symbols-rounded">
            {isRunning ? 'power_settings_new' : 'play_arrow'}
          </span>
          {isRunning ? '停止服务端' : '一键启动服务'}
        </button>
      </div>

      {#if isRunning && localUrl}
        <div class="url-box">
          <span class="label">手机端访问地址</span>
          <div class="url-row">
            <span class="url-text">{localUrl}/api/status</span>
            <button class="icon-btn" aria-label="复制" onclick={handleCopy}>
              <span class="material-symbols-rounded">content_copy</span>
            </button>
            <a href="{localUrl}/api/screenshot" target="_blank" class="icon-btn link-btn" aria-label="查看截图接口">
              <span class="material-symbols-rounded">open_in_new</span>
            </a>
          </div>
        </div>
      {/if}
    </div>

    <div class="settings-card">
      <h4>服务设置</h4>
      <div class="setting-row">
        <div class="setting-info">
          <span class="material-symbols-rounded">visibility_off</span>
          <div>
            <h5>隐私模糊模式</h5>
            <p>开启后，远程获取的屏幕截图将被极速多重模糊处理，防止文字泄露。</p>
          </div>
        </div>
        <button 
          class="switch-btn" 
          class:active={isPrivacyEnabled} 
          onclick={handleTogglePrivacy}
        >
          <div class="knob"></div>
        </button>
      </div>
    </div>
    
  </div>
</div>

<style>
  .tool-view {
    display: flex;
    flex-direction: column;
    gap: 24px;
    height: 100%;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
  }

  .back-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: fit-content;
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    transition: color var(--transition-fast);
  }
  .back-btn:hover { color: var(--text-primary); }
  .back-btn .material-symbols-rounded { font-size: 18px; }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
  }

  .header-left {
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
    color: #0A84FF;
    background-color: rgba(10, 132, 255, 0.15);
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
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .content-area {
    display: flex;
    flex-direction: column;
    gap: 24px;
    flex: 1;
    min-height: 0;
  }

  .main-card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 32px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    box-shadow: var(--shadow-md);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 10px;
    background-color: var(--bg-panel1);
    padding: 8px 16px;
    border-radius: var(--radius-full);
    border: 1px solid var(--border-subtle);
  }

  .pulse-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background-color: var(--text-secondary);
    transition: background-color 0.3s;
  }

  .status-indicator.running .pulse-dot {
    background-color: #34C759;
    box-shadow: 0 0 12px rgba(52, 199, 89, 0.6);
    animation: pulse 2s infinite;
  }

  .status-text {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .status-indicator.running .status-text {
    color: #34C759;
  }

  @keyframes pulse {
    0% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(52, 199, 89, 0.7); }
    70% { transform: scale(1); box-shadow: 0 0 0 6px rgba(52, 199, 89, 0); }
    100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(52, 199, 89, 0); }
  }

  .desc {
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-secondary);
    text-align: center;
    max-width: 480px;
  }

  .power-section {
    display: flex;
    justify-content: center;
    width: 100%;
  }

  .power-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 16px 40px;
    font-size: 18px;
    font-weight: 600;
    color: white;
    background: linear-gradient(135deg, #0A84FF, #005bb5);
    border: none;
    border-radius: var(--radius-full);
    cursor: pointer;
    box-shadow: 0 8px 24px rgba(10, 132, 255, 0.3);
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .power-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 28px rgba(10, 132, 255, 0.4);
  }

  .power-btn:active {
    transform: translateY(1px);
  }

  .power-btn.on {
    background: linear-gradient(135deg, #FF3B30, #b50000);
    box-shadow: 0 8px 24px rgba(255, 59, 48, 0.3);
  }

  .power-btn.on:hover {
    box-shadow: 0 12px 28px rgba(255, 59, 48, 0.4);
  }

  .url-box {
    margin-top: 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: 100%;
    max-width: 480px;
  }

  .url-box .label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-caption);
    text-transform: uppercase;
  }

  .url-row {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-panel1);
    padding: 12px 16px;
    border-radius: var(--radius-md);
    border: 1px solid rgba(10, 132, 255, 0.3);
    width: 100%;
  }

  .url-text {
    flex: 1;
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 14px;
    color: #0A84FF;
    font-weight: 600;
  }

  .icon-btn {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--text-secondary);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    background-color: var(--bg-panel0);
    color: var(--text-primary);
  }

  .icon-btn .material-symbols-rounded {
    font-size: 18px;
  }

  .link-btn {
    text-decoration: none;
  }

  .settings-card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 24px;
    box-shadow: var(--shadow-sm);
  }

  .settings-card h4 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 20px;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    background-color: var(--bg-panel1);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
  }

  .setting-info {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .setting-info .material-symbols-rounded {
    font-size: 28px;
    color: var(--text-secondary);
  }

  .setting-info h5 {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .setting-info p {
    font-size: 13px;
    color: var(--text-caption);
  }

  .switch-btn {
    position: relative;
    width: 50px;
    height: 30px;
    background-color: var(--border-focus);
    border-radius: 15px;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  .switch-btn.active {
    background-color: #34C759;
  }

  .knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 26px;
    height: 26px;
    background-color: white;
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .switch-btn.active .knob {
    transform: translateX(20px);
  }
</style>
