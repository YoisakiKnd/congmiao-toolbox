<script lang="ts">
  import { appState } from '../state.svelte';
  import init, { Workspace } from '@astral-sh/ruff-wasm-web';
  import { onMount } from 'svelte';

  let inputData = $state('');
  let outputData = $state('');
  let errorMessage = $state('');
  let formatMode = $state<'format' | 'struct'>('format');
  let ruffWorkspace = $state<Workspace | null>(null);
  let initError = $state('');

  onMount(async () => {
    try {
      await init();
      ruffWorkspace = new Workspace(Workspace.defaultSettings(), 0);
    } catch (e: any) {
      console.error('Failed to load ruff wasm:', e);
      initError = 'Ruff WASM 加载失败，格式化不可用';
    }
  });

  const formatPythonStruct = (input: string) => {
    let jsonStr = input
      .replace(/\bTrue\b/g, 'true')
      .replace(/\bFalse\b/g, 'false')
      .replace(/\bNone\b/g, 'null');
      
    jsonStr = jsonStr.replace(/'([^'\\]*(?:\\.[^'\\]*)*)'/g, '"$1"');
    
    try {
      const obj = JSON.parse(jsonStr);
      return JSON.stringify(obj, null, 2);
    } catch (e: any) {
      throw new Error('解析失败：输入可能包含复杂的 Python 语法。');
    }
  };

  const handleFormat = () => {
    errorMessage = '';
    outputData = '';
    
    if (!inputData.trim()) return;

    try {
      if (formatMode === 'struct') {
        outputData = formatPythonStruct(inputData);
      } else if (formatMode === 'format') {
        if (!ruffWorkspace) {
          throw new Error('Ruff WASM 引擎未加载');
        }
        outputData = ruffWorkspace.format(inputData);
      }
    } catch (e: any) {
      errorMessage = e.message || '格式化失败';
    }
  };

  const handleCopy = async () => {
    if (!outputData) return;
    try {
      await navigator.clipboard.writeText(outputData);
    } catch (err) {
      console.error('Failed to copy', err);
    }
  };
</script>

<div class="formatter-container">
  <button class="back-btn" onclick={() => appState.activeToolId = null}>
    <span class="material-symbols-rounded">arrow_back</span>
    <span>全部功能</span>
  </button>

  <div class="header">
    <div class="header-left">
      <div class="icon blue">
        <span class="material-symbols-rounded">data_object</span>
      </div>
      <div>
        <p class="eyebrow">开发工具</p>
        <h3>Python 工具集</h3>
      </div>
    </div>

    <div class="mode-toggle">
      <button 
        class="toggle-btn {formatMode === 'format' ? 'active' : ''}" 
        onclick={() => { formatMode = 'format'; handleFormat(); }}
      >
        代码排版 (Ruff)
      </button>
      <button 
        class="toggle-btn {formatMode === 'struct' ? 'active' : ''}" 
        onclick={() => { formatMode = 'struct'; handleFormat(); }}
      >
        字典转 JSON
      </button>
    </div>
  </div>

  {#if initError}
    <div class="init-error">
      <span class="material-symbols-rounded">warning</span>
      {initError}
    </div>
  {/if}

  <div class="editor-area">
    <div class="pane">
      <div class="pane-header">
        <span class="label">输入代码片段 👇</span>
      </div>
      <textarea 
        class="code-input" 
        bind:value={inputData} 
        oninput={handleFormat}
        placeholder={formatMode === 'format' ? '粘贴混乱的 Python 代码...' : '粘贴带有单引号、True、False 的字典...'}
      ></textarea>
      {#if errorMessage}
        <div class="error-toast">
          <span class="material-symbols-rounded">error</span>
          <span>{errorMessage}</span>
        </div>
      {/if}
    </div>

    <div class="pane">
      <div class="pane-header">
        <span class="label">输出结果</span>
        <button class="icon-action-btn" aria-label="Copy Output" onclick={handleCopy}>
          <span class="material-symbols-rounded">content_copy</span>
        </button>
      </div>
      <textarea 
        class="code-output" 
        readonly 
        value={outputData}
        placeholder={formatMode === 'format' ? '排版后的优美代码...' : '转化后的标准 JSON 将在这里显示...'}
      ></textarea>
    </div>
  </div>
</div>

<style>
  .formatter-container {
    display: flex;
    flex-direction: column;
    gap: 24px;
    height: 100%;
    max-width: 1600px;
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

  .mode-toggle {
    display: flex;
    background-color: var(--bg-panel0);
    padding: 4px;
    border-radius: 8px;
    border: 1px solid var(--border-subtle);
  }

  .toggle-btn {
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 600;
    border-radius: 6px;
    color: var(--text-secondary);
    transition: all var(--transition-fast);
  }

  .toggle-btn.active {
    background-color: var(--bg-app);
    color: var(--text-primary);
    box-shadow: var(--shadow-sm);
  }

  .init-error {
    background-color: rgba(255, 149, 0, 0.1);
    color: #FF9500;
    padding: 12px 16px;
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 600;
  }

  .editor-area {
    display: flex;
    gap: 24px;
    flex: 1;
    min-height: 0;
  }

  .pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    overflow: hidden;
    position: relative;
    box-shadow: var(--shadow-sm);
  }

  .pane-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--bg-panel1);
    border-bottom: 1px solid var(--border-subtle);
  }

  .label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-caption);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .icon-action-btn {
    color: var(--text-secondary);
    transition: color var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .icon-action-btn:hover { color: var(--text-primary); }
  .icon-action-btn .material-symbols-rounded { font-size: 18px; }

  .code-input, .code-output {
    flex: 1;
    width: 100%;
    padding: 16px;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 13px;
    line-height: 1.5;
    resize: none;
    outline: none;
  }

  .code-output {
    background-color: var(--bg-app);
  }

  .error-toast {
    position: absolute;
    bottom: 24px;
    left: 16px;
    right: 16px;
    background-color: rgba(255, 59, 48, 0.9);
    backdrop-filter: blur(10px);
    color: #fff;
    padding: 12px 16px;
    border-radius: 8px;
    display: flex;
    align-items: flex-start;
    gap: 8px;
    font-size: 13px;
    box-shadow: 0 4px 12px rgba(255, 59, 48, 0.2);
    animation: fadeIn 0.2s ease-out;
  }
  .error-toast .material-symbols-rounded { font-size: 18px; }
</style>
