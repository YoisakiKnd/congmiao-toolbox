<script lang="ts">
  import Sidebar from './components/Sidebar.svelte';
  import Topbar from './components/Topbar.svelte';
  import CommandPalette from './components/CommandPalette.svelte';
  import SystemMonitorTile from './components/SystemMonitorTile.svelte';
  import PeekPCWidget from './components/PeekPCWidget.svelte';
  import JsonFormatter from './components/JsonFormatter.svelte';
  import PythonFormatter from './components/PythonFormatter.svelte';
  import EncoderTool from './components/EncoderTool.svelte';
  import ColorPickerTool from './components/ColorPickerTool.svelte';
  import HashTool from './components/HashTool.svelte';
  import ImageConverterTool from './components/ImageConverterTool.svelte';
  import TimerTool from './components/TimerTool.svelte';
  import TranslatorTool from './components/TranslatorTool.svelte';
  import PeekPCTool from './components/PeekPCTool.svelte';
  import ScreenTimeView from './components/ScreenTimeView.svelte';
  import HeartRateWidget from './components/HeartRateWidget.svelte';
  import HROverlay from './components/HROverlay.svelte';
  import { appState } from './state.svelte';
  import { runTool } from './tools';
  import { onMount } from 'svelte';

  let isOverlay = $state(false);

  onMount(() => {
    // Check if this window is the OBS overlay
    if (window.location.hash === '#/hr-overlay') {
      isOverlay = true;
      return; // Skip main app initialization for overlay
    }

    document.documentElement.setAttribute('data-theme', appState.theme);

    const handleKeydown = (event: KeyboardEvent) => {
      const withMeta = event.metaKey || event.ctrlKey;

      if (withMeta && event.key.toLowerCase() === 'k') {
        event.preventDefault();
        appState.commandOpen = true;
      }

      if (withMeta && event.key.toLowerCase() === 't') {
        event.preventDefault();
        runTool('timestamp');
      }

      if (withMeta && event.key.toLowerCase() === 'j') {
        event.preventDefault();
        runTool('json-format');
      }
    };

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if isOverlay}
  <HROverlay />
{:else}
<div class="app-layout">
  <Sidebar />
  
  <main class="main-content">
    <Topbar />
    
    <div class="workspace">
      {#if appState.activeNavIndex === 0}
        <!-- 仪表盘 (Dashboard) -->
        <div class="bento-grid">
          <SystemMonitorTile />
          <PeekPCWidget />
          <HeartRateWidget />
        </div>
      {:else if appState.activeNavIndex === 1}
        <!-- 全部工具 (All Tools) -->
        {#if appState.activeToolId === 'json'}
          <div class="full-page-tool"><JsonFormatter /></div>
        {:else if appState.activeToolId === 'python'}
          <div class="full-page-tool"><PythonFormatter /></div>
        {:else if appState.activeToolId === 'encoder'}
          <div class="full-page-tool"><EncoderTool /></div>
        {:else if appState.activeToolId === 'color'}
          <div class="full-page-tool"><ColorPickerTool /></div>
        {:else if appState.activeToolId === 'hash'}
          <div class="full-page-tool"><HashTool /></div>
        {:else if appState.activeToolId === 'image'}
          <div class="full-page-tool"><ImageConverterTool /></div>
        {:else if appState.activeToolId === 'timer'}
          <div class="full-page-tool"><TimerTool /></div>
        {:else if appState.activeToolId === 'translator'}
          <div class="full-page-tool"><TranslatorTool /></div>
        {:else if appState.activeToolId === 'peek_pc'}
          <div class="full-page-tool"><PeekPCTool /></div>
        {:else}
          <div class="tool-directory">
            <div class="dir-header">
              <h3>全部功能大厅</h3>
              <p class="subtitle">选择一个工具开始使用</p>
            </div>
            <div class="dir-grid">
              <button class="tool-card" onclick={() => appState.activeToolId = 'json'}>
                <div class="icon orange"><span class="material-symbols-rounded">data_object</span></div>
                <div class="info"><h4>JSON 格式化</h4><p>美化 JSON 文本并查错</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'python'}>
                <div class="icon blue"><span class="material-symbols-rounded">data_object</span></div>
                <div class="info"><h4>Python 工具集</h4><p>代码极速排版与字典转换</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'encoder'}>
                <div class="icon green"><span class="material-symbols-rounded">swap_horiz</span></div>
                <div class="info"><h4>万能编码转换</h4><p>Base64, URL, 与 Unicode 互转</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'color'}>
                <div class="icon pink"><span class="material-symbols-rounded">colorize</span></div>
                <div class="info"><h4>深层取色器</h4><p>系统级屏幕取色工具 RGB/HEX</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'hash'}>
                <div class="icon purple"><span class="material-symbols-rounded">fingerprint</span></div>
                <div class="info"><h4>哈希校验中心</h4><p>极速并发计算 MD5, SHA 系列</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'image'}>
                <div class="icon orange"><span class="material-symbols-rounded">imagesmode</span></div>
                <div class="info"><h4>图片格式工厂</h4><p>离线高品质转存 PNG/JPG/WEBP</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'timer'}>
                <div class="icon red"><span class="material-symbols-rounded">timer</span></div>
                <div class="info"><h4>生产力时钟</h4><p>支持计圈的秒表与倒计时器</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'translator'}>
                <div class="icon teal"><span class="material-symbols-rounded">translate</span></div>
                <div class="info"><h4>多语互译机</h4><p>六国语言极速免费互译助手</p></div>
              </button>
              <button class="tool-card" onclick={() => appState.activeToolId = 'peek_pc'}>
                <div class="icon blue"><span class="material-symbols-rounded">desktop_windows</span></div>
                <div class="info"><h4>Peek 远程监视</h4><p>局域网跨屏监视硬件状态</p></div>
              </button>
            </div>
          </div>
        {/if}
      {:else if appState.activeNavIndex === 2}
        <!-- 使用时长 (Screen Time) -->
        <ScreenTimeView />
      {/if}
    </div>
  </main>
</div>

<CommandPalette />
{/if}

<style>
  .tool-directory {
    display: flex;
    flex-direction: column;
    gap: 32px;
    padding-top: 16px;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
  }

  .dir-header h3 {
    font-size: 28px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 8px;
    letter-spacing: -0.02em;
  }

  .dir-header .subtitle {
    font-size: 15px;
    color: var(--text-secondary);
  }

  .dir-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
    gap: 20px;
  }

  .tool-card {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    padding: 24px;
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    text-align: left;
    transition: all var(--transition-fast);
  }

  .tool-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-sm);
    border-color: var(--border-focus);
    background-color: var(--bg-panel-hover);
  }

  .tool-card .icon {
    display: grid;
    place-items: center;
    width: 48px;
    height: 48px;
    border-radius: 12px;
    flex-shrink: 0;
  }

  .tool-card .icon.orange { color: #FF9500; background-color: rgba(255, 149, 0, 0.15); }
  .tool-card .icon.blue { color: #0A84FF; background-color: rgba(10, 132, 255, 0.15); }
  .tool-card .icon.green { color: #34C759; background-color: rgba(52, 199, 89, 0.15); }
  .tool-card .icon.pink { color: #FF2D55; background-color: rgba(255, 45, 85, 0.15); }
  .tool-card .icon.purple { color: #AF52DE; background-color: rgba(175, 82, 222, 0.15); }
  .tool-card .icon.red { color: #FF3B30; background-color: rgba(255, 59, 48, 0.15); }
  .tool-card .icon.teal { color: #30B0C7; background-color: rgba(48, 176, 199, 0.15); }

  .tool-card .info {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .tool-card .info h4 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .tool-card .info p {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
  }
  .app-layout {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background-color: var(--bg-app);
  }

  .main-content {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    margin-left: 80px;
  }

  .workspace {
    flex: 1;
    padding: 32px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .bento-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 20px;
    width: 100%;
    margin: 0 auto;
    max-width: 1000px;
    align-content: start;
    flex: 1;
  }

  .full-page-tool {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
</style>
