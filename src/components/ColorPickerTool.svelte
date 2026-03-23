<script lang="ts">
  import { appState } from '../state.svelte';

  let selectedColor = $state('#0A84FF');
  let eyeDropperSupported = $state(false);

  import { onMount } from 'svelte';
  onMount(() => {
    // @ts-ignore
    eyeDropperSupported = window.EyeDropper !== undefined;
  });

  const triggerEyeDropper = async () => {
    try {
      // @ts-ignore
      const eyeDropper = new window.EyeDropper();
      const result = await eyeDropper.open();
      selectedColor = result.sRGBHex;
    } catch (e) {
      console.log('User canceled eye dropper');
    }
  };

  // Convert HEX to RGB
  const hexToRgb = (hex: string) => {
    let raw = hex.replace('#', '');
    if (raw.length === 3) raw = raw.split('').map(c => c + c).join('');
    const bigint = parseInt(raw, 16);
    const r = (bigint >> 16) & 255;
    const g = (bigint >> 8) & 255;
    const b = bigint & 255;
    return { r, g, b };
  };

  // Convert RGB to HSL
  const rgbToHsl = (r: number, g: number, b: number) => {
    r /= 255; g /= 255; b /= 255;
    let max = Math.max(r, g, b), min = Math.min(r, g, b);
    let h = 0, s = 0, l = (max + min) / 2;

    if (max !== min) {
      let d = max - min;
      s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
      switch (max) {
        case r: h = (g - b) / d + (g < b ? 6 : 0); break;
        case g: h = (b - r) / d + 2; break;
        case b: h = (r - g) / d + 4; break;
      }
      h /= 6;
    }
    return { 
      h: Math.round(h * 360), 
      s: Math.round(s * 100), 
      l: Math.round(l * 100) 
    };
  };

  const rgb = $derived(hexToRgb(selectedColor));
  const hsl = $derived(rgbToHsl(rgb.r, rgb.g, rgb.b));
  
  const rgbString = $derived(`rgb(${rgb.r}, ${rgb.g}, ${rgb.b})`);
  const hslString = $derived(`hsl(${hsl.h}, ${hsl.s}%, ${hsl.l}%)`);
  const hexString = $derived(selectedColor.toUpperCase());

  const handleCopy = async (text: string) => {
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
      <div class="icon pink">
        <span class="material-symbols-rounded">colorize</span>
      </div>
      <div>
        <p class="eyebrow">设计助手</p>
        <h3>原生取色器</h3>
      </div>
    </div>
  </div>

  <div class="content-area">
    
    <!-- Color Display & Input Box -->
    <div class="color-preview-box">
      <div class="color-swatch" style="background-color: {selectedColor}">
        <input type="color" bind:value={selectedColor} class="native-color-picker" aria-label="Choose color" />
        <div class="click-hint">
          <span class="material-symbols-rounded">touch_app</span>
          <span>点击调出系统控制台</span>
        </div>
      </div>
      
      {#if eyeDropperSupported}
      <button class="eyedropper-btn" onclick={triggerEyeDropper}>
        <span class="material-symbols-rounded">colorize</span>
        吸取屏幕任意颜色
      </button>
      {/if}
    </div>

    <!-- Values Grid -->
    <div class="values-grid">
      <div class="value-card">
        <span class="label">HEX</span>
        <div class="val-box">
          <span class="val-text">{hexString}</span>
          <button class="icon-btn" onclick={() => handleCopy(hexString)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
      </div>
      <div class="value-card">
        <span class="label">RGB</span>
        <div class="val-box">
          <span class="val-text">{rgbString}</span>
          <button class="icon-btn" onclick={() => handleCopy(rgbString)}><span class="material-symbols-rounded">content_copy</span></button>
        </div>
      </div>
      <div class="value-card">
        <span class="label">HSL</span>
        <div class="val-box">
          <span class="val-text">{hslString}</span>
          <button class="icon-btn" onclick={() => handleCopy(hslString)}><span class="material-symbols-rounded">content_copy</span></button>
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
    max-width: 1000px;
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

  .icon.pink {
    color: #FF2D55;
    background-color: rgba(255, 45, 85, 0.15);
    border-color: rgba(255, 45, 85, 0.2);
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
    margin-top: 16px;
    align-items: flex-start;
  }

  .color-preview-box {
    display: flex;
    flex-direction: column;
    gap: 16px;
    width: 300px;
  }

  .color-swatch {
    width: 100%;
    height: 240px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-subtle);
    box-shadow: var(--shadow-md);
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: flex-end;
    padding: 16px;
    transition: background-color 0.2s ease;
  }

  .native-color-picker {
    position: absolute;
    top: -100px;
    left: -100px;
    width: 200%;
    height: 200%;
    cursor: pointer;
    opacity: 0.001; /* Must not be exactly 0 to work in some browsers */
  }

  .click-hint {
    background-color: rgba(0, 0, 0, 0.5);
    color: white;
    padding: 8px 12px;
    border-radius: 8px;
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 500;
    pointer-events: none;
    transition: opacity 0.2s;
  }

  .color-swatch:hover .click-hint {
    opacity: 0;
  }

  .eyedropper-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    color: var(--text-primary);
    font-size: 14px;
    font-weight: 600;
    padding: 14px;
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    box-shadow: var(--shadow-sm);
  }

  .eyedropper-btn:hover {
    background-color: var(--bg-panel-hover);
    border-color: var(--border-focus);
    transform: translateY(-2px);
  }

  .values-grid {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .value-card {
    background-color: var(--bg-panel0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 16px 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: var(--shadow-sm);
  }

  .label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .val-box {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--bg-panel1);
    padding: 12px 16px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
  }

  .val-text {
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: 0.02em;
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
