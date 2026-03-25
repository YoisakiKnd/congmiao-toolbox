<script lang="ts">
  import { getVersion } from '@tauri-apps/api/app';
  import { ask, message } from '@tauri-apps/plugin-dialog';
  import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
  import { relaunch } from '@tauri-apps/plugin-process';
  import { check, type Update } from '@tauri-apps/plugin-updater';
  import { onDestroy, onMount } from 'svelte';
  import { appState } from '../state.svelte';

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

  let autostartEnabled = $state<boolean | null>(null);
  let autostartBusy = $state(false);
  let updateBusy = $state(false);
  let updatePendingVersion = $state<string | null>(null);
  let updateStatusText = $state('检查更新');
  let pendingUpdate = $state<Update | null>(null);

  const updateIcon = $derived(() => {
    if (updateBusy) return 'progress_activity';
    if (updatePendingVersion) return 'download';
    return 'system_update';
  });

  const updateButtonLabel = $derived(() => {
    if (updateBusy) return updateStatusText;
    if (updatePendingVersion) return `更新 ${updatePendingVersion}`;
    return updateStatusText;
  });

  const closePendingUpdate = async () => {
    if (!pendingUpdate) return;

    try {
      await pendingUpdate.close();
    } catch (error) {
      console.error(error);
    }

    pendingUpdate = null;
  };

  const refreshDesktopState = async () => {
    appState.appVersion = await getVersion();
    autostartEnabled = await isEnabled();
  };

  const syncUpdate = async (interactive = false) => {
    if (updateBusy) return;
    updateBusy = true;
    updateStatusText = '检查中...';

    try {
      await closePendingUpdate();
      const update = await check();

      if (update) {
        pendingUpdate = update;
        updatePendingVersion = update.version;
        updateStatusText = '可安装更新';

        appState.addActivity({
          source: 'SYSTEM',
          title: 'Update Ready',
          value: `发现新版本 v${update.version}`,
          accent: 'teal',
        });

        if (interactive) {
          await installPendingUpdate();
        }
      } else {
        updatePendingVersion = null;
        updateStatusText = '已是最新';

        if (interactive) {
          await message(`当前已经是最新版本 v${appState.appVersion}。`, {
            title: '检查更新',
            kind: 'info',
          });
        }
      }
    } catch (error) {
      console.error(error);
      updatePendingVersion = null;
      updateStatusText = interactive ? '更新失败' : '检查更新';

      if (interactive) {
        await message('检查更新失败，请稍后再试。', {
          title: '检查更新',
          kind: 'error',
        });
      }
    } finally {
      updateBusy = false;
    }
  };

  const installPendingUpdate = async () => {
    if (!pendingUpdate || updateBusy) return;

    const shouldInstall = await ask(
      `发现新版本 v${pendingUpdate.version}，要现在下载并安装吗？`,
      {
        title: '版本更新',
        kind: 'info',
        okLabel: '立即更新',
        cancelLabel: '稍后再说',
      }
    );

    if (!shouldInstall) return;

    updateBusy = true;
    updateStatusText = '下载中...';

    try {
      await pendingUpdate.downloadAndInstall((event) => {
        if (event.event === 'Started') updateStatusText = '开始下载';
        if (event.event === 'Progress') updateStatusText = '正在下载';
        if (event.event === 'Finished') updateStatusText = '正在安装';
      });

      appState.addActivity({
        source: 'SYSTEM',
        title: 'Update Installed',
        value: `已安装 v${pendingUpdate.version}`,
        accent: 'teal',
      });

      const shouldRestart = await ask(
        '更新已经安装完成，是否现在重启应用？',
        {
          title: '版本更新',
          kind: 'info',
          okLabel: '立即重启',
          cancelLabel: '稍后手动重启',
        }
      );

      updatePendingVersion = null;
      updateStatusText = shouldRestart ? '正在重启' : '重启后生效';
      await closePendingUpdate();

      if (shouldRestart) {
        await relaunch();
      }
    } catch (error) {
      console.error(error);
      updateStatusText = '安装失败';
      await message('下载或安装更新失败，请稍后再试。', {
        title: '版本更新',
        kind: 'error',
      });
    } finally {
      updateBusy = false;
    }
  };

  const handleUpdateClick = async () => {
    if (updatePendingVersion) {
      await installPendingUpdate();
      return;
    }

    await syncUpdate(true);
  };

  const toggleAutostart = async () => {
    if (autostartBusy || autostartEnabled === null) return;

    autostartBusy = true;

    try {
      if (autostartEnabled) {
        await disable();
        autostartEnabled = false;
        appState.addActivity({
          source: 'SYSTEM',
          title: 'Autostart Disabled',
          value: '已关闭开机自启',
          accent: 'blue',
        });
      } else {
        await enable();
        autostartEnabled = true;
        appState.addActivity({
          source: 'SYSTEM',
          title: 'Autostart Enabled',
          value: '已开启开机自启',
          accent: 'blue',
        });
      }
    } catch (error) {
      console.error(error);
      await message('切换开机自启失败，请稍后再试。', {
        title: '开机自启',
        kind: 'error',
      });
    } finally {
      autostartBusy = false;
    }
  };

  onMount(async () => {
    await refreshDesktopState();
    await syncUpdate(false);
  });

  onDestroy(() => {
    void closePendingUpdate();
  });
</script>

<header class="topbar">
  <div class="topbar-title">
    <p class="eyebrow">{currentDate()}</p>
    <h2>{currentTime()}</h2>
  </div>

  <div class="topbar-actions">
    <button
      class="status-chip"
      class:active={autostartEnabled === true}
      disabled={autostartBusy}
      title={autostartEnabled ? '关闭开机自启' : '开启开机自启'}
      aria-label={autostartEnabled ? '关闭开机自启' : '开启开机自启'}
      onclick={toggleAutostart}
    >
      <span class="material-symbols-rounded">rocket_launch</span>
      <span>{autostartEnabled ? '已启用自启' : '开机自启'}</span>
    </button>

    <button
      class="status-chip"
      class:highlight={updatePendingVersion !== null}
      disabled={updateBusy}
      title={updatePendingVersion ? `安装 v${updatePendingVersion}` : '检查更新'}
      aria-label={updatePendingVersion ? `安装 v${updatePendingVersion}` : '检查更新'}
      onclick={handleUpdateClick}
    >
      <span class="material-symbols-rounded">{updateIcon()}</span>
      <span>{updateButtonLabel()}</span>
    </button>

    <div class="version-chip">v{appState.appVersion}</div>

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
    gap: 16px;
    min-height: 80px;
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
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .status-chip,
  .version-chip {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    min-height: 36px;
    padding: 0 14px;
    border-radius: 999px;
    border: 1px solid var(--border-subtle);
    background-color: var(--bg-panel0);
    font-size: 12px;
    font-weight: 700;
    color: var(--text-secondary);
    transition: all var(--transition-fast);
  }

  .status-chip:hover {
    background-color: var(--bg-panel-hover);
    color: var(--text-primary);
    border-color: var(--border-focus);
  }

  .status-chip.active {
    color: #1b8f4d;
    border-color: rgba(52, 199, 89, 0.35);
    background-color: rgba(52, 199, 89, 0.12);
  }

  .status-chip.highlight {
    color: #0A84FF;
    border-color: rgba(10, 132, 255, 0.35);
    background-color: rgba(10, 132, 255, 0.12);
  }

  .status-chip .material-symbols-rounded {
    font-size: 18px;
  }

  .version-chip {
    color: var(--text-caption);
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

  @media (max-width: 980px) {
    .topbar {
      align-items: flex-start;
      padding: 20px 24px;
    }

    .topbar-actions {
      max-width: 55%;
    }
  }
</style>
