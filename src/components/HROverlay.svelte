<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';

  let bpm = $state(0);
  let unlisten: (() => void) | null = null;
  let pollTimer: ReturnType<typeof setInterval>;
  let useGreen = $state(true);

  let pulseInterval = $derived(bpm > 0 ? (60000 / bpm) : 1000);

  let errorMsg = $state('');

  onMount(async () => {
    // Listen for real-time HR events
    try {
      unlisten = await listen<number>('hr-update', (event) => {
        bpm = event.payload;
      });
    } catch (e) {
      errorMsg = `Listen Error: ${e}`;
    }

    // Aggressive polling as primary data source for the overlay window
    const poll = async () => {
      try {
        const s = await invoke<{bpm: number; connected: boolean}>('get_hr_status');
        if (s.bpm > 0) bpm = s.bpm;
      } catch (e) {
        errorMsg = `Invoke Error: ${e}`;
      }
    };
    await poll();
    pollTimer = setInterval(poll, 1000);
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (pollTimer) clearInterval(pollTimer);
  });

  const toggleBg = () => { useGreen = !useGreen; };
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" class:green={useGreen} ondblclick={toggleBg}>
  <div class="content">
    <!-- SVG Dynamic Heart -->
    <svg
      class="heart-svg"
      class:beating={bpm > 0}
      style="animation-duration: {pulseInterval}ms"
      viewBox="0 0 24 24"
      fill="currentColor"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
    </svg>

    <div class="data-col">
      <span class="bpm-number">{bpm > 0 ? bpm : '--'}</span>
      <span class="bpm-label">BPM</span>
    </div>
  </div>

  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}

  <p class="hint">双击切换背景</p>
</div>

<style>
  :global(html), :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    height: 100%;
    width: 100%;
  }

  .overlay {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100vw;
    height: 100vh;
    background: #1a1a1a;
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    user-select: none;
    cursor: default;
    transition: background-color 0.3s;
  }

  .overlay.green {
    background: #00b140;
  }

  .content {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .heart-svg {
    width: 64px;
    height: 64px;
    color: #FF3B30;
    filter: drop-shadow(0 0 12px rgba(255, 59, 48, 0.5));
    flex-shrink: 0;
  }

  .heart-svg.beating {
    animation: heartbeat 1s ease-in-out infinite;
  }

  @keyframes heartbeat {
    0%, 100% { transform: scale(1); }
    12% { transform: scale(1.25); }
    24% { transform: scale(0.95); }
    36% { transform: scale(1.15); }
    48% { transform: scale(1); }
  }

  .data-col {
    display: flex;
    align-items: baseline;
    gap: 6px;
  }

  .bpm-number {
    font-size: 64px;
    font-weight: 800;
    color: white;
    letter-spacing: -0.04em;
    line-height: 1;
    text-shadow: 0 2px 16px rgba(0, 0, 0, 0.3);
  }

  .bpm-label {
    font-size: 22px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.6);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .error {
    position: absolute;
    top: 8px;
    font-size: 10px;
    color: #FF3B30;
    background: rgba(0,0,0,0.5);
    padding: 2px 6px;
    border-radius: 4px;
  }

  .hint {
    position: absolute;
    bottom: 8px;
    font-size: 10px;
    color: rgba(255, 255, 255, 0.3);
    margin: 0;
    pointer-events: none;
  }

  .overlay.green .hint { color: rgba(0, 0, 0, 0.25); }
  .overlay.green .bpm-number { text-shadow: 0 2px 16px rgba(0, 0, 0, 0.15); }
</style>
