<script lang="ts">
  import { appState } from '../state.svelte';
  import SparkMD5 from 'spark-md5';

  let textInput = $state('');
  let fileObj = $state<File | null>(null);
  let isCalculating = $state(false);
  let dragHover = $state(false);
  
  let hashes = $state({
    md5: '',
    sha1: '',
    sha256: '',
    sha512: ''
  });

  const arrayBufferToHex = (buffer: ArrayBuffer) => {
    return Array.from(new Uint8Array(buffer))
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  };

  const calculateHashes = async (buffer: ArrayBuffer) => {
    isCalculating = true;
    try {
      hashes.md5 = SparkMD5.ArrayBuffer.hash(buffer);
      hashes.sha1 = arrayBufferToHex(await crypto.subtle.digest('SHA-1', buffer));
      hashes.sha256 = arrayBufferToHex(await crypto.subtle.digest('SHA-256', buffer));
      // hashes.sha512 = arrayBufferToHex(await crypto.subtle.digest('SHA-512', buffer)); // Optional, keeping it simple
    } catch (e) {
      console.error(e);
    }
    isCalculating = false;
  };

  const handleTextInput = async () => {
    fileObj = null;
    if (!textInput) {
      hashes = { md5: '', sha1: '', sha256: '', sha512: '' };
      return;
    }
    const encoder = new TextEncoder();
    const buffer = encoder.encode(textInput).buffer;
    await calculateHashes(buffer.slice(0)); // copy buffer
  };

  const handleDragOver = (e: DragEvent) => { e.preventDefault(); dragHover = true; };
  const handleDragLeave = (e: DragEvent) => { e.preventDefault(); dragHover = false; };

  const processFile = async (file: File) => {
    if (!file) return;
    textInput = '';
    fileObj = file;
    isCalculating = true;
    // For large files, use FileReader chunking in a real production app.
    // For this toolkit, arrayBuffer() is fast enough for files < 500MB
    const arrayBuffer = await file.arrayBuffer();
    await calculateHashes(arrayBuffer);
  };

  const handleFileDrop = async (e: DragEvent) => {
    e.preventDefault();
    dragHover = false;
    const file = e.dataTransfer?.files[0];
    if (file) await processFile(file);
  };

  const handleFileInput = async (e: Event) => {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      await processFile(target.files[0]);
    }
  };

  const handleCopy = async (text: string) => {
    if (!text) return;
    try { await navigator.clipboard.writeText(text); } 
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
      <div class="icon purple">
        <span class="material-symbols-rounded">fingerprint</span>
      </div>
      <div>
        <p class="eyebrow">安全与校验</p>
        <h3>哈希/MD5 校验计算</h3>
      </div>
    </div>
  </div>

  <div class="content-area">
    <!-- Left: Input/Drop Area -->
    <div class="input-section">
      <div class="pane">
        <div class="pane-header">
          <span class="label">输入文本</span>
        </div>
        <textarea 
          class="code-input" 
          bind:value={textInput} 
          oninput={handleTextInput}
          placeholder="输入需要计算 Hash 的英文字符串或文本..."
        ></textarea>
      </div>

      <div class="drop-zone" 
           class:hover={dragHover}
           ondragover={handleDragOver}
           ondragleave={handleDragLeave}
           ondrop={handleFileDrop}>
        <span class="material-symbols-rounded drop-icon">upload_file</span>
        <div class="drop-texts">
          <h4>拖拽文件校验计算</h4>
          <p>或者点击此处选择文件</p>
        </div>
        <input type="file" class="file-input" onchange={handleFileInput} />
      </div>

      {#if fileObj}
        <div class="file-info">
          <span class="material-symbols-rounded">description</span>
          <div class="file-meta">
            <span class="filename">{fileObj.name}</span>
            <span class="filesize">{(fileObj.size / 1024 / 1024).toFixed(2)} MB</span>
          </div>
        </div>
      {/if}
    </div>

    <!-- Right: Output Grids -->
    <div class="output-grid">
      {#if isCalculating}
        <div class="loading-state">
          <span class="material-symbols-rounded spinner">sync</span>
          <p>正在极速计算...</p>
        </div>
      {:else}
        <div class="result-card">
          <div class="rc-header">
            <h4>MD5</h4>
            <span class="badge">常用于文件校验</span>
          </div>
          <div class="rc-body">
            <input type="text" readonly value={hashes.md5} placeholder="..." />
            <button class="icon-btn" onclick={() => handleCopy(hashes.md5)}><span class="material-symbols-rounded">content_copy</span></button>
          </div>
        </div>

        <div class="result-card">
          <div class="rc-header">
            <h4>SHA-1</h4>
            <span class="badge">Git / 历史遗留版本校验</span>
          </div>
          <div class="rc-body">
            <input type="text" readonly value={hashes.sha1} placeholder="..." />
            <button class="icon-btn" onclick={() => handleCopy(hashes.sha1)}><span class="material-symbols-rounded">content_copy</span></button>
          </div>
        </div>

        <div class="result-card">
          <div class="rc-header">
            <h4>SHA-256</h4>
            <span class="badge blue">高安全标准 (推荐)</span>
          </div>
          <div class="rc-body">
            <input type="text" readonly value={hashes.sha256} placeholder="..." />
            <button class="icon-btn" onclick={() => handleCopy(hashes.sha256)}><span class="material-symbols-rounded">content_copy</span></button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .tool-view {
    display: flex;
    flex-direction: column;
    gap: 24px;
    height: 100%;
    max-width: 1400px;
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

  .icon.purple {
    color: #AF52DE;
    background-color: rgba(175, 82, 222, 0.15);
    border-color: rgba(175, 82, 222, 0.2);
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
    gap: 32px;
    flex: 1;
    min-height: 0;
  }

  .input-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .pane {
    display: flex;
    flex-direction: column;
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    overflow: hidden;
    height: 200px;
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

  .drop-zone {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    background-color: var(--bg-panel0);
    border: 2px dashed var(--border-focus);
    border-radius: var(--radius-lg);
    position: relative;
    cursor: pointer;
    transition: all 0.2s;
  }

  .drop-zone:hover, .drop-zone.hover {
    background-color: var(--bg-panel-hover);
    border-color: #0A84FF;
  }

  .file-input {
    position: absolute;
    top: 0; left: 0; width: 100%; height: 100%;
    opacity: 0;
    cursor: pointer;
  }

  .drop-icon {
    font-size: 48px;
    color: var(--text-caption);
    transition: color 0.2s;
  }

  .drop-zone:hover .drop-icon {
    color: #0A84FF;
  }

  .drop-texts h4 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
    text-align: center;
  }

  .drop-texts p {
    font-size: 13px;
    color: var(--text-caption);
    text-align: center;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    background-color: rgba(10, 132, 255, 0.1);
    border: 1px solid rgba(10, 132, 255, 0.2);
    border-radius: var(--radius-md);
    color: #0A84FF;
  }

  .file-meta {
    display: flex;
    flex-direction: column;
  }

  .filename {
    font-size: 14px;
    font-weight: 600;
  }

  .filesize {
    font-size: 12px;
    opacity: 0.8;
  }

  .output-grid {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow-y: auto;
  }

  .result-card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: var(--shadow-sm);
  }

  .rc-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .rc-header h4 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .badge {
    font-size: 10px;
    font-weight: 600;
    background-color: var(--bg-panel1);
    color: var(--text-secondary);
    padding: 4px 8px;
    border-radius: var(--radius-full);
    text-transform: uppercase;
  }

  .badge.blue {
    background-color: rgba(10, 132, 255, 0.15);
    color: #0A84FF;
  }

  .rc-body {
    display: flex;
    align-items: center;
    gap: 12px;
    background-color: var(--bg-panel1);
    border-radius: var(--radius-md);
    padding: 4px;
    border: 1px solid var(--border-subtle);
  }

  .rc-body input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 13px;
    padding: 8px 12px;
    outline: none;
  }

  .icon-btn {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: 8px;
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

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-secondary);
    gap: 16px;
  }

  .spinner {
    font-size: 32px;
    animation: spin 1s linear infinite;
  }

  @keyframes spin { 100% { transform: rotate(360deg); } }
</style>
