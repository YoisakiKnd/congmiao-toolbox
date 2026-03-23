<script lang="ts">
  import { appState } from '../state.svelte';

  let inputData = $state('');
  let outputData = $state('');
  let errorMessage = $state('');

  const handleFormat = () => {
    errorMessage = '';
    outputData = '';
    
    if (!inputData.trim()) return;

    try {
      const obj = JSON.parse(inputData);
      outputData = JSON.stringify(obj, null, 2);
    } catch (e: any) {
      errorMessage = '无效的 JSON: ' + e.message;
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
      <div class="icon orange">
        <span class="material-symbols-rounded">data_object</span>
      </div>
      <div>
        <p class="eyebrow">开发工具</p>
        <h3>JSON 格式化</h3>
      </div>
    </div>
  </div>

  <div class="editor-area">
    <div class="pane">
      <div class="pane-header">
        <span class="label">输入 JSON</span>
      </div>
      <textarea 
        class="code-input" 
        bind:value={inputData} 
        oninput={handleFormat}
        placeholder="粘贴 JSON 字符串..."
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
        <span class="label">格式化输出</span>
        <button class="icon-action-btn" aria-label="Copy Output" onclick={handleCopy}>
          <span class="material-symbols-rounded">content_copy</span>
        </button>
      </div>
      <textarea 
        class="code-output" 
        readonly 
        value={outputData}
        placeholder="结果..."
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

  .icon.orange {
    color: #FF9500;
    background-color: rgba(255, 149, 0, 0.15);
    border-color: rgba(255, 149, 0, 0.2);
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
