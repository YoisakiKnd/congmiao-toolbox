<script lang="ts">
  import { appState, navItems } from '../state.svelte';
</script>

{#if !appState.sidebarCollapsed}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="sidebar-backdrop" onclick={() => appState.sidebarCollapsed = true}></div>
{/if}

<aside class="sidebar {appState.sidebarCollapsed ? 'collapsed' : ''}">
  <div class="sidebar-top">
    <div class="brand">
      {#if !appState.sidebarCollapsed}
        <div class="brand-copy">
          <p class="eyebrow">Congmiao Toolbox</p>
          <h1>Console</h1>
        </div>
      {/if}

      <button
        class="toggle-btn"
        aria-label={appState.sidebarCollapsed ? '展开侧栏' : '折叠侧栏'}
        onclick={() => appState.sidebarCollapsed = !appState.sidebarCollapsed}
      >
        <span class="material-symbols-rounded">
          {appState.sidebarCollapsed ? 'right_panel_open' : 'left_panel_close'}
        </span>
      </button>
    </div>

    <nav class="nav-list">
      {#each navItems as item, index}
        <button
          class="nav-item {appState.activeNavIndex === index ? 'active' : ''}"
          onclick={() => appState.activeNavIndex = index}
          title={appState.sidebarCollapsed ? item.title : ''}
        >
          <div class="nav-icon">
            <span class="material-symbols-rounded">{item.icon}</span>
          </div>
          {#if !appState.sidebarCollapsed}
            <div class="nav-text">
              <span class="nav-title">{item.title}</span>
              <span class="nav-caption">{item.caption}</span>
            </div>
          {/if}
        </button>
      {/each}
    </nav>
  </div>

  <div class="sidebar-bottom">
    {#if appState.sidebarCollapsed}
      <button class="nav-item" onclick={() => appState.commandOpen = true} title="搜索工具 (Cmd+K)">
        <div class="nav-icon">
          <span class="material-symbols-rounded">search</span>
        </div>
      </button>
    {:else}
      <button class="command-launch" onclick={() => appState.commandOpen = true}>
        <span class="material-symbols-rounded">search</span>
        <span>搜索工具 (Cmd+K)</span>
      </button>
      <div class="version">v0.1.0</div>
    {/if}
  </div>
</aside>

<style>
  .sidebar-backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(4px);
    animation: fadeIn 0.2s ease-out;
  }

  .sidebar {
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    z-index: 50;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    width: 280px;
    background-color: rgba(26, 26, 26, 0.85);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border-right: 1px solid var(--border-subtle);
    padding: 24px 16px;
    transition: all 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  :global([data-theme="light"]) .sidebar {
    background-color: rgba(255, 255, 255, 0.85);
  }

  .sidebar.collapsed {
    width: 80px;
    padding: 24px 12px;
    background-color: var(--bg-sidebar);
    backdrop-filter: none;
    -webkit-backdrop-filter: none;
  }

  .sidebar-top {
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .brand {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 48px;
    padding-left: 8px;
  }

  .brand-copy {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .eyebrow {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-caption);
  }

  h1 {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }

  .toggle-btn {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    transition: all var(--transition-fast);
  }

  .toggle-btn:hover {
    background-color: var(--bg-panel-hover);
    color: var(--text-primary);
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    text-align: left;
    width: 100%;
  }

  .nav-item:hover {
    background-color: var(--bg-panel-hover);
  }

  .nav-item.active {
    background-color: var(--bg-panel1);
    box-shadow: inset 0 0 0 1px var(--border-focus);
  }

  .nav-icon {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background-color: transparent;
    color: var(--text-secondary);
    flex-shrink: 0;
    transition: all var(--transition-fast);
  }

  .nav-icon .material-symbols-rounded {
    font-size: 20px;
  }

  .nav-item.active .nav-icon {
    background-color: var(--accent-blue-soft);
    color: var(--accent-blue);
  }

  .nav-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
  }

  .nav-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .nav-caption {
    font-size: 12px;
    color: var(--text-caption);
  }

  .sidebar-bottom {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin-top: auto;
  }

  .command-launch {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    margin-top: 24px;
    padding: 12px;
    border-radius: var(--radius-md);
    background-color: var(--bg-panel0);
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    border: 1px solid var(--border-subtle);
    transition: all var(--transition-fast);
  }

  .command-launch:hover {
    background-color: var(--bg-panel-hover);
    color: var(--text-primary);
    border-color: var(--border-focus);
  }

  .command-launch .material-symbols-rounded {
    font-size: 18px;
  }

  .version {
    font-size: 11px;
    color: var(--text-caption);
    font-weight: 600;
    text-align: center;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
