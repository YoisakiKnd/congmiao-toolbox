<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  interface MediaInfo {
    title: string;
    artist: string;
    album: string;
    playing: boolean;
  }


  interface AudioStatus {
    volume: number;
    muted: boolean;
    media: MediaInfo | null;
  }

  let isRunning = $state(false);
  let isPrivacyEnabled = $state(false);
  let privacyImagePath = $state<string | null>(null);
  let isToggling = $state(false);
  let localIp = $state('0.0.0.0');
  let statusTimer: ReturnType<typeof setInterval>;

  let audioStatus = $state<AudioStatus>({ volume: 0, muted: false, media: null });

  const checkStatus = async () => {
    isRunning = await invoke<boolean>('get_peek_status');
    isPrivacyEnabled = await invoke<boolean>('get_privacy_status');
    // We assume there's a way to get the audio status too
    // In our Axum API it is /api/status, 
    // but let's just use a simple fetch here if running
    if (isRunning) {
      try {
        const resp = await fetch(`http://localhost:3000/api/status`);
        const data = await resp.json();
        audioStatus = data.audio;
      } catch (e) { console.error(e); }
    }
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
        localIp = '0.0.0.0';
      } else {
        localIp = await invoke<string>('start_peek_server');
        isRunning = true;
      }
    } catch (e) { console.error(e); }
    isToggling = false;
  };

  const handleTogglePrivacy = async () => {
    isPrivacyEnabled = await invoke<boolean>('toggle_privacy');
  };

  const handleSelectImage = async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }]
    });
    if (selected && typeof selected === 'string') {
      privacyImagePath = selected;
      await invoke('set_peek_privacy_image', { path: selected });
    }
  };

  const handleClearImage = async () => {
    privacyImagePath = null;
    await invoke('set_peek_privacy_image', { path: null });
  };

  const handleToggleMute = async () => {
    const newMute = !audioStatus.muted;
    // We can call our Axum API or a Tauri command. 
    // Let's use the Axum API to demonstrate the "remote" control capability.
    try {
      await fetch(`http://localhost:3000/api/audio`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mute: newMute })
      });
      audioStatus.muted = newMute;
    } catch (e) {
      console.error(e);
    }
  };
</script>

<div class="tool-content">
  <div class="tool-header">
    <div class="icon blue">
      <span class="material-symbols-rounded">cast</span>
    </div>
    <div class="title-meta">
      <h3>Peek PC 遥控监控</h3>
      <p>在局域网内通过浏览器或其他设备监控本机屏幕。{isRunning ? '服务已启动' : '服务未运行'}</p>
    </div>
  </div>

  <div class="control-grid">
    <!-- Server Status Card -->
    <div class="card status-card" class:active={isRunning}>
      <div class="card-title">
        <span class="material-symbols-rounded">sensors</span>
        服务状态
      </div>
      <div class="ip-display">
        <span class="label">访问地址</span>
        <code>http://{localIp}:3000</code>
      </div>
      <button class="toggle-btn" onclick={handleToggleServer}>
        <span class="material-symbols-rounded">{isRunning ? 'stop_circle' : 'play_circle'}</span>
        {isRunning ? '停止服务端' : '启动服务端'}
      </button>
    </div>

    <!-- Privacy Settings Card -->
    <div class="card settings-card">
      <div class="card-title">
        <span class="material-symbols-rounded">visibility_off</span>
        隐私模式 (高强度 80% 模糊)
      </div>
      <div class="setting-row">
        <span>启用隐私遮罩</span>
        <button class="switch" class:on={isPrivacyEnabled} onclick={handleTogglePrivacy} title="切换隐私遮罩" aria-label="切换隐私遮罩">
          <div class="knob"></div>
        </button>
      </div>

      
      <div class="image-selector">
        <span class="label">自定义隐私图片</span>
        {#if privacyImagePath}
          <div class="path-box">
            <span class="path">{privacyImagePath.split('/').pop()}</span>
            <button class="clear-btn" onclick={handleClearImage}>
              <span class="material-symbols-rounded">close</span>
            </button>
          </div>
        {:else}
          <button class="outline-btn" onclick={handleSelectImage}>
            <span class="material-symbols-rounded">add_photo_alternate</span>
            选择图片
          </button>
        {/if}
      </div>
    </div>

    <!-- Audio Settings Card -->
    <div class="card settings-card">
      <div class="card-title">
        <span class="material-symbols-rounded">volume_up</span>
        音频控制 (Win/Mac)
      </div>
      <div class="setting-row">
        <span>系统静音</span>
        <button class="switch" class:on={audioStatus.muted} onclick={handleToggleMute} title="切换静音" aria-label="切换静音">
          <div class="knob"></div>
        </button>
      </div>

      <div class="volume-info">
        <span class="label">当前音量</span>
        <div class="vol-bar">
          <div class="vol-fill" style="width: {audioStatus.volume}%"></div>
        </div>
        <span class="vol-num">{audioStatus.volume}%</span>
      </div>
    </div>

    <!-- Media Info Card -->
    {#if audioStatus.media}
      <div class="card media-card">
        <div class="card-title">
          <span class="material-symbols-rounded">music_note</span>
          正在播放
        </div>
        <div class="media-content">
          <div class="media-info">
            <span class="song-title">{audioStatus.media.title}</span>
            <span class="artist-name">{audioStatus.media.artist}</span>
            <span class="album-name">{audioStatus.media.album}</span>
          </div>
          <div class="play-state" class:playing={audioStatus.media.playing}>
            <span class="material-symbols-rounded">
              {audioStatus.media.playing ? 'pause_circle' : 'play_circle'}
            </span>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <div class="tip-box">
    <span class="material-symbols-rounded">info</span>
    <p>隐私模式会对截图进行多重下采样并执行 30.0 径向模糊，确保关键文本无法辨认。</p>
  </div>
</div>

<style>
  .tool-content {
    display: flex;
    flex-direction: column;
    gap: 32px;
    max-width: 900px;
  }

  .tool-header {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .icon {
    display: grid;
    place-items: center;
    width: 48px;
    height: 48px;
    border-radius: 14px;
  }

  .icon.blue {
    background-color: rgba(10, 132, 255, 0.15);
    color: #0A84FF;
  }

  .title-meta h3 { margin: 0; font-size: 20px; font-weight: 700; color: var(--text-primary); }
  .title-meta p { margin: 4px 0 0; font-size: 14px; color: var(--text-secondary); }

  .control-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
  }

  .card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: 20px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .card-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .card-title .material-symbols-rounded { font-size: 18px; color: var(--text-secondary); }

  .ip-display {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .label { font-size: 12px; font-weight: 600; color: var(--text-caption); text-transform: uppercase; }
  code {
    background: var(--bg-panel1);
    padding: 12px;
    border-radius: 12px;
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 14px;
    color: #0A84FF;
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 14px;
    background-color: #0A84FF;
    color: white;
    border: none;
    border-radius: 14px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s;
  }

  .toggle-btn:hover { background-color: #0070e0; transform: translateY(-1px); }
  .status-card.active .toggle-btn { background-color: rgba(255, 59, 48, 0.1); color: #FF3B30; }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .switch {
    position: relative;
    width: 44px;
    height: 24px;
    background-color: var(--bg-panel1);
    border-radius: 12px;
    border: 1px solid var(--border-subtle);
    cursor: pointer;
    transition: 0.3s cubic-bezier(0.18, 0.89, 0.32, 1.28);
  }

  .switch.on { background-color: #34C759; border-color: #34C759; }

  .knob {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 16px;
    height: 16px;
    background-color: white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0,0,0,0.2);
    transition: transform 0.3s cubic-bezier(0.18, 0.89, 0.32, 1.28);
  }

  .switch.on .knob { transform: translateX(20px); }

  .outline-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px;
    background: transparent;
    border: 1px dashed var(--border-subtle);
    border-radius: 12px;
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .outline-btn:hover { background: var(--bg-panel1); border-color: var(--border-focus); color: var(--text-primary); }

  .path-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--bg-panel1);
    padding: 8px 12px;
    border-radius: 10px;
    font-size: 13px;
  }

  .path { color: var(--text-primary); font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 180px; }

  .clear-btn { background: transparent; border: none; color: var(--text-caption); cursor: pointer; display: flex; }
  .clear-btn:hover { color: #FF3B30; }

  .volume-info { display: flex; flex-direction: column; gap: 8px; }
  .vol-bar { height: 6px; background: var(--bg-panel1); border-radius: 3px; overflow: hidden; }
  .vol-fill { height: 100%; background: #0A84FF; border-radius: 3px; transition: width 0.3s ease; }
  .vol-num { font-size: 12px; font-weight: 700; color: var(--text-primary); align-self: flex-end; }

  .media-card { background: linear-gradient(135deg, var(--bg-panel0) 0%, rgba(10, 132, 255, 0.05) 100%); }
  .media-content { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  .media-info { display: flex; flex-direction: column; gap: 2px; overflow: hidden; }
  .song-title { font-size: 15px; font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .artist-name { font-size: 13px; font-weight: 500; color: var(--text-secondary); }
  .album-name { font-size: 11px; color: var(--text-caption); }
  
  .play-state { color: #0A84FF; }
  .play-state.playing { color: #34C759; }
  .play-state .material-symbols-rounded { font-size: 32px; }

  .tip-box {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    background-color: rgba(10, 132, 255, 0.05);
    border-radius: 16px;
    border: 1px solid rgba(10, 132, 255, 0.1);
  }

  .tip-box .material-symbols-rounded { color: #0A84FF; font-size: 20px; }
  .tip-box p { margin: 0; font-size: 13px; color: var(--text-secondary); line-height: 1.5; }
</style>
