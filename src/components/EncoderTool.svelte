<script lang="ts">
  import { appState } from '../state.svelte';

  let inputData = $state('');

  // Computed results
  const urlEncoded = $derived(encodeURIComponent(inputData));
  
  let urlDecoded = $derived.by(() => {
    if (!inputData) return '';
    try { return decodeURIComponent(inputData); } 
    catch { return '解析失败（包含非标准 URL 编码字符）'; }
  });

  let base64Encoded = $derived.by(() => {
    if (!inputData) return '';
    try { 
      // Handle unicode base64
      return btoa(encodeURIComponent(inputData).replace(/%([0-9A-F]{2})/g,
        function toSolidBytes(match, p1) {
            return String.fromCharCode(Number('0x' + p1));
        }));
    } catch { return '编码失败'; }
  });

  let base64Decoded = $derived.by(() => {
    if (!inputData) return '';
    try {
      return decodeURIComponent(atob(inputData).split('').map(function(c) {
          return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
      }).join(''));
    } catch { return '非有效的 Base64 文本'; }
  });

  const unicodeEscaped = $derived.by(() => {
    if (!inputData) return '';
    return inputData.split('').map(char => {
      const code = char.charCodeAt(0);
      return code > 127 ? '\\u' + code.toString(16).padStart(4, '0') : char;
    }).join('');
  });

  let unicodeUnescaped = $derived.by(() => {
    if (!inputData) return '';
    try {
      return inputData.replace(/\\u([0-9a-fA-F]{4})/g, (match, grp) => {
        return String.fromCharCode(parseInt(grp, 16));
      });
    } catch { return '反转义失败'; }
  });

  const handleCopy = async (text: string) => {
    if (!text || text.includes('失败') || text.includes('非有效')) return;
    try {
      await navigator.clipboard.writeText(text);
    } catch (e) { console.error('Copy failed'); }
  };
</script>

<div class="tool-view">
  <button class="back-btn" onclick={() => appState.activeToolId = null}>
    <span class="material-symbols-rounded">arrow_back</span>
    <span>全部功能</span>
  </button>

  <div class="header">
    <div class="header-left">
      <div class="icon green">
        <span class="material-symbols-rounded">swap_horiz</span>
      </div>
      <div>
        <p class="eyebrow">转换工具</p>
        <h3>智能编解码专家</h3>
      </div>
    </div>
  </div>

  <div class="editor-area">
    <!-- Left: Input Area -->
    <div class="pane input-pane">
      <div class="pane-header">
        <span class="label">输入源文本 👇</span>
      </div>
      <textarea 
        class="code-input" 
        bind:value={inputData} 
        placeholder="粘贴任何需要转换的文本、Base64 或 URL..."
      ></textarea>
    </div>

    <!-- Right: Output Grids -->
    <div class="output-grid">
      <!-- Base64 Block -->
      <div class="output-block">
        <div class="block-header">
          <span>Base64 编码与解码</span>
        </div>
        <div class="result-row">
          <span class="result-label">Encode</span>
          <input type="text" readonly value={base64Encoded} />
          <button class="icon-btn" onclick={() => handleCopy(base64Encoded)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
        <div class="result-row">
          <span class="result-label">Decode</span>
          <input type="text" readonly value={base64Decoded} class:error={base64Decoded.includes('非有效')} />
          <button class="icon-btn" onclick={() => handleCopy(base64Decoded)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
      </div>

      <!-- URL Block -->
      <div class="output-block">
        <div class="block-header">
          <span>URL 编码与解码</span>
        </div>
        <div class="result-row">
          <span class="result-label">Encode</span>
          <input type="text" readonly value={urlEncoded} />
          <button class="icon-btn" onclick={() => handleCopy(urlEncoded)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
        <div class="result-row">
          <span class="result-label">Decode</span>
          <input type="text" readonly value={urlDecoded} class:error={urlDecoded.includes('解析失败')} />
          <button class="icon-btn" onclick={() => handleCopy(urlDecoded)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
      </div>

      <!-- Unicode Block -->
      <div class="output-block">
        <div class="block-header">
          <span>Unicode 中文转换</span>
        </div>
        <div class="result-row">
          <span class="result-label">中文转 \u</span>
          <input type="text" readonly value={unicodeEscaped} />
          <button class="icon-btn" onclick={() => handleCopy(unicodeEscaped)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
        <div class="result-row">
          <span class="result-label">\u 转中文</span>
          <input type="text" readonly value={unicodeUnescaped} />
          <button class="icon-btn" onclick={() => handleCopy(unicodeUnescaped)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
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

  .icon.green {
    color: #34C759;
    background-color: rgba(52, 199, 89, 0.15);
    border-color: rgba(52, 199, 89, 0.2);
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
    display: flex;
    flex-direction: column;
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    overflow: hidden;
    position: relative;
    box-shadow: var(--shadow-sm);
  }

  .input-pane {
    flex: 1;
    max-width: 40%;
  }

  .output-grid {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow-y: auto;
    padding-right: 8px;
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

  .code-input {
    flex: 1;
    width: 100%;
    padding: 16px;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-size: 14px;
    line-height: 1.6;
    resize: none;
    outline: none;
  }

  .output-block {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: var(--shadow-sm);
  }

  .block-header {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .result-row {
    display: flex;
    align-items: center;
    gap: 12px;
    background-color: var(--bg-panel1);
    border-radius: var(--radius-md);
    padding: 4px;
    border: 1px solid var(--border-subtle);
  }

  .result-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-caption);
    text-transform: uppercase;
    width: 60px;
    text-align: right;
    padding-left: 8px;
  }

  .result-row input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 13px;
    padding: 8px;
    outline: none;
  }

  .result-row input.error {
    color: #FF3B30;
  }

  .icon-btn {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--text-secondary);
    transition: all 0.2s;
  }

  .icon-btn:hover {
    background-color: var(--bg-panel0);
    color: var(--text-primary);
  }

  .icon-btn .material-symbols-rounded {
    font-size: 18px;
  }
</style>
