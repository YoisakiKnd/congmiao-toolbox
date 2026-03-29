<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '../state.svelte';

  interface WheelCategory {
    name: string;
    items?: string[];
    subCategories?: WheelCategory[];
    color?: string;
  }

  // State
  let categories = $state<WheelCategory[]>([
    { 
      name: '爆破地图', 
      items: ['404基地', '88区', '欧拉港口', '风曳镇', '空间实验室', '柯西街区', '科斯迷特', '奥卡努斯'],
      color: '#FF3B30' 
    },
    { 
      name: '角色', 
      subCategories: [
        { 
          name: '欧泊', 
          items: ['米雪儿·李', '信', '芙拉薇娅', '忧雾', '蕾欧娜', '心夏', '千代', '伊薇特'],
          color: '#007AFF'
        },
        { 
          name: '乌尔比诺', 
          items: ['奥黛丽', '玛德蕾娜', '白墨', '绯莎', '汐', '星绘', '加拉蒂亚'],
          color: '#5856D6'
        },
        { 
          name: '剪刀手', 
          items: ['令', '艾卡', '珐格兰丝', '玛拉', '拉薇', '明', '梅瑞狄斯', '香奈美'],
          color: '#AF52DE'
        }
      ],
      color: '#34C759'
    },
    { 
      name: '副武器', 
      items: ['小蜜蜂', '重焰', '雪鸮', '焚焰者'],
      color: '#FF9500' 
    }
  ]);

  // Navigation History
  let navPath = $state<WheelCategory[]>([]);
  
  let currentCategory = $derived(
    navPath.length === 0 ? null : navPath[navPath.length - 1]
  );

  let currentOptions = $derived(
    currentCategory === null 
      ? categories.map(c => c.name) 
      : (currentCategory.subCategories 
          ? currentCategory.subCategories.map(sc => sc.name) 
          : (currentCategory.items || []))
  );

  let isSpinning = $state(false);
  let rotation = $state(0);
  let result = $state<string | null>(null);
  let importText = $state('');
  let showImportModal = $state(false);

  let segmentAngle = $derived(currentOptions.length > 0 ? 360 / currentOptions.length : 0);

  const spin = () => {
    if (isSpinning || currentOptions.length === 0) return;
    isSpinning = true;
    result = null;

    const extraSpins = 5 + Math.random() * 5;
    const randomAngle = Math.random() * 360;
    const targetRotation = rotation + (extraSpins * 360) + randomAngle;
    
    rotation = targetRotation;

    setTimeout(() => {
      isSpinning = false;
      // CORRECT CALCULATION:
      // Item 0 is at top (-90deg). Rotation is clockwise.
      // The item at top post-rotation is the one whose relative angle was (360 - rotation % 360)
      const normalizedAngle = (targetRotation % 360);
      const pointerAngleRelativeToWheel = (360 - normalizedAngle) % 360;
      const index = Math.floor(pointerAngleRelativeToWheel / segmentAngle) % currentOptions.length;
      
      result = currentOptions[index];

      appState.addActivity({
        source: 'TEXT',
        title: `转盘结果：${result}`,
        value: currentCategory ? `在 ${currentCategory.name} 中` : '大分类',
        accent: 'blue'
      });
    }, 4000);
  };

  const enterCategory = (name: string) => {
    // Look into current level
    let next: WheelCategory | undefined;
    if (currentCategory === null) {
      next = categories.find(c => c.name === name);
    } else if (currentCategory.subCategories) {
      next = currentCategory.subCategories.find(sc => sc.name === name);
    }

    if (next) {
      navPath = [...navPath, next];
      result = null;
      rotation = 0;
    }
  };

  const backNav = () => {
    navPath = navPath.slice(0, -1);
    result = null;
    rotation = 0;
  };

  const handleNextStep = () => {
    if (result) {
      enterCategory(result);
    }
  };

  const generateColors = (n: number) => {
    const palette = ['#FF3B30', '#FF9500', '#FFCC00', '#34C759', '#007AFF', '#5856D6', '#AF52DE', '#FF2D55'];
    return Array.from({ length: n }, (_, i) => palette[i % palette.length]);
  };

  let optionColors = $derived(generateColors(currentOptions.length));

  const handleImport = () => {
    try {
      const parsed = JSON.parse(importText);
      if (Array.isArray(parsed)) {
        categories = parsed;
        navPath = [];
        showImportModal = false;
        importText = '';
      }
    } catch (e) {
      alert('导入失败，请检查 JSON 格式');
    }
  };
</script>

<div class="tool-content">
  <div class="tool-header">
    <div class="icon purple">
      <span class="material-symbols-rounded">cyclone</span>
    </div>
    <div class="title-meta">
      <h3>幸运大转盘</h3>
      <p>
        {#if navPath.length === 0}
          正在决定大类
        {:else}
          {navPath.map(n => n.name).join(' > ')}
        {/if}
      </p>
    </div>
    <div class="header-actions">
      {#if navPath.length > 0}
        <button class="text-btn" onclick={backNav}>
          <span class="material-symbols-rounded">arrow_back</span>
          返回上一级
        </button>
      {/if}
      <button class="outline-btn" onclick={() => showImportModal = true}>
        <span class="material-symbols-rounded">file_upload</span>
        导入预设
      </button>
    </div>
  </div>

  <div class="main-layout">
    <div class="wheel-container">
      <div class="pointer"></div>
      <div class="wheel-wrapper" style="transform: rotate({rotation}deg); transition: transform 4s cubic-bezier(0.15, 0, 0.15, 1);">
        <svg viewBox="0 0 100 100">
          <defs>
            <filter id="inner-shadow">
              <feOffset dx="0" dy="2" />
              <feGaussianBlur stdDeviation="1" result="offset-blur" />
              <feComposite operator="out" in="SourceGraphic" in2="offset-blur" result="inverse" />
              <feFlood flood-color="black" flood-opacity="0.2" result="color" />
              <feComposite operator="in" in="color" in2="inverse" result="shadow" />
              <feComposite operator="over" in="shadow" in2="SourceGraphic" />
            </filter>
          </defs>
          {#each currentOptions as option, i}
            {@const startAngle = i * segmentAngle - 90}
            {@const endAngle = (i + 1) * segmentAngle - 90}
            <path
              d="M 50 50 L {50 + 50 * Math.cos(startAngle * Math.PI / 180)} {50 + 50 * Math.sin(startAngle * Math.PI / 180)} A 50 50 0 0 1 {50 + 50 * Math.cos(endAngle * Math.PI / 180)} {50 + 50 * Math.sin(endAngle * Math.PI / 180)} Z"
              fill={optionColors[i]}
              stroke="rgba(255,255,255,0.3)"
              stroke-width="0.5"
            />
            <text
              x="50"
              y="22"
              transform="rotate({i * segmentAngle + segmentAngle / 2}, 50, 50)"
              fill="white"
              font-size={currentOptions.length > 12 ? '2.5' : '3.5'}
              font-weight="800"
              text-anchor="middle"
              style="text-shadow: 0 1px 2px rgba(0,0,0,0.3)"
            >
              {option}
            </text>
          {/each}
        </svg>
      </div>
      
      <button class="spin-center" class:disabled={isSpinning} onclick={spin}>
        {isSpinning ? '...' : (currentCategory?.subCategories || navPath.length === 0 ? 'ROLL 类' : 'ROLL !')}
      </button>
    </div>

    <div class="control-panel">
      {#if result}
        <div class="result-card fade-in">
          <div class="label">本次结果</div>
          <div class="value">{result}</div>
          
          {#if currentCategory === null || currentCategory.subCategories}
             <!-- Check if result is a sub-category -->
             <button class="action-btn" onclick={handleNextStep}>
               进入 "{result}" 继续下一级
             </button>
          {/if}
        </div>
      {/if}

      <div class="quick-nav">
        <div class="section-title">直达分类</div>
        <div class="nav-stack">
          <button class="nav-item" class:active={currentCategory === null} onclick={() => navPath = []}>
            根目录 (大类)
          </button>
          {#each categories as cat}
             {#if cat.subCategories}
               <div class="sub-group">
                 <button class="nav-item" class:active={currentCategory === cat} onclick={() => navPath = [cat]}>
                    {cat.name}
                 </button>
                 <div class="child-list">
                    {#each cat.subCategories as sub}
                      <button class="child-item" class:active={currentCategory === sub} onclick={() => navPath = [cat, sub]}>
                        ↳ {sub.name}
                      </button>
                    {/each}
                 </div>
               </div>
             {:else}
                <button class="nav-item" class:active={currentCategory === cat} onclick={() => navPath = [cat]}>
                  {cat.name}
                </button>
             {/if}
          {/each}
        </div>
      </div>
    </div>
  </div>

  {#if showImportModal}
    <div 
      class="modal-overlay" 
      role="button" 
      tabindex="-1"
      onclick={() => showImportModal = false}
      onkeydown={e => e.key === 'Escape' && (showImportModal = false)}
    >
      <div 
        class="modal-content" 
        role="dialog"
        aria-modal="true"
        tabindex="-1"
        onclick={e => e.stopPropagation()}
        onkeydown={e => e.stopPropagation()}
      >
        <h3>导入多级决策预设</h3>
        <p>支持无限级嵌套 JSON。格式示例： [&lbrace;"name":"A","subCategories":[...]&rbrace;] 或 [&lbrace;"name":"B","items":[...]&rbrace;]</p>
        <textarea 
          bind:value={importText} 
          placeholder='[&lbrace;"name":"分类","items":["选项1"]&rbrace;]'
        ></textarea>
        <div class="modal-actions">
          <button class="text-btn" onclick={() => showImportModal = false}>取消</button>
          <button class="primary-btn" onclick={handleImport}>确认导入</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .tool-content { display: flex; flex-direction: column; gap: 32px; max-width: 1000px; }
  .tool-header { display: flex; align-items: center; gap: 20px; }
  .icon.purple { background-color: rgba(175, 82, 222, 0.15); color: #AF52DE; width: 48px; height: 48px; border-radius: 14px; display: grid; place-items: center; }
  .title-meta h3 { margin: 0; font-size: 20px; font-weight: 700; color: var(--text-primary); }
  .title-meta p { margin: 4px 0 0; font-size: 14px; color: var(--text-secondary); }
  .header-actions { margin-left: auto; display: flex; gap: 12px; }

  .main-layout { display: grid; grid-template-columns: 1fr 320px; gap: 40px; align-items: start; }

  .wheel-container { position: relative; width: 100%; aspect-ratio: 1; max-width: 500px; margin: 0 auto; }
  .wheel-wrapper { width: 100%; height: 100%; filter: drop-shadow(0 12px 40px rgba(0,0,0,0.15)); }
  .pointer { 
    position: absolute; 
    top: -12px; 
    left: 50%; 
    transform: translateX(-50%); 
    width: 32px; 
    height: 44px; 
    background: #FF3B30; 
    clip-path: polygon(50% 100%, 0 0, 100% 0);
    z-index: 10;
    filter: drop-shadow(0 4px 8px rgba(0,0,0,0.3));
    border-radius: 4px;
  }

  .spin-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 84px;
    height: 84px;
    background: white;
    border: 8px solid var(--bg-panel0);
    border-radius: 50%;
    color: #1a1a1a;
    font-weight: 900;
    font-size: 14px;
    cursor: pointer;
    z-index: 5;
    box-shadow: 0 8px 24px rgba(0,0,0,0.25);
    transition: all 0.2s cubic-bezier(0.18, 0.89, 0.32, 1.28);
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    line-height: 1.2;
  }
  .spin-center:hover:not(.disabled) { transform: translate(-50%, -50%) scale(1.1); box-shadow: 0 12px 32px rgba(0,0,0,0.3); }
  .spin-center.disabled { opacity: 0.6; cursor: not-allowed; }

  .control-panel { display: flex; flex-direction: column; gap: 24px; }
  .result-card { 
    background: linear-gradient(135deg, #AF52DE 0%, #5856D6 100%); 
    padding: 24px; 
    border-radius: 20px; 
    color: white; 
    text-align: center;
    box-shadow: 0 12px 32px rgba(175, 82, 222, 0.4);
  }
  .result-card .label { font-size: 11px; font-weight: 800; opacity: 0.8; margin-bottom: 8px; text-transform: uppercase; }
  .result-card .value { font-size: 36px; font-weight: 900; margin-bottom: 16px; letter-spacing: -0.02em; }
  .action-btn { 
    width: 100%;
    background: rgba(255,255,255,0.2); 
    border: 1px solid rgba(255,255,255,0.4); 
    color: white; 
    padding: 12px; 
    border-radius: 12px; 
    font-size: 14px; 
    font-weight: 700;
    cursor: pointer;
    transition: 0.2s;
  }
  .action-btn:hover { background: rgba(255,255,255,0.35); }

  .quick-nav { background: var(--bg-panel0); border-radius: 20px; border: 1px solid var(--border-subtle); padding: 20px; }
  .section-title { font-size: 14px; font-weight: 700; margin-bottom: 16px; color: var(--text-primary); opacity: 0.8; }
  
  .nav-stack { display: flex; flex-direction: column; gap: 6px; }
  .nav-item { 
    text-align: left; background: var(--bg-panel1); border: 1px solid transparent; 
    padding: 12px 16px; border-radius: 12px; font-size: 14px; font-weight: 600;
    color: var(--text-secondary); cursor: pointer; transition: 0.2s;
  }
  .nav-item.active { background: rgba(175, 82, 222, 0.1); border-color: #AF52DE; color: #AF52DE; }
  .nav-item:hover:not(.active) { background: var(--bg-panel-hover); }

  .sub-group { display: flex; flex-direction: column; gap: 4px; border-left: 2px solid var(--border-subtle); margin-left: 8px; }
  .child-list { display: flex; flex-direction: column; gap: 2px; padding-left: 12px; }
  .child-item { 
    text-align: left; background: transparent; border: none; padding: 10px 12px; 
    border-radius: 8px; font-size: 13px; color: var(--text-caption); cursor: pointer;
  }
  .child-item.active { color: #AF52DE; font-weight: 700; background: rgba(175, 82, 222, 0.05); }
  .child-item:hover { color: var(--text-primary); background: var(--bg-panel-hover); }

  /* Modal & Others same as before */
  .modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.4); backdrop-filter: blur(8px); display: grid; place-items: center; z-index: 1000; }
  .modal-content { background: var(--bg-panel0); padding: 32px; border-radius: 28px; width: 90%; max-width: 500px; box-shadow: 0 24px 80px rgba(0,0,0,0.4); border: 1px solid var(--border-subtle); }
  textarea { width: 100%; height: 240px; margin: 16px 0; padding: 16px; background: var(--bg-panel1); border: 1px solid var(--border-subtle); border-radius: 16px; font-family: ui-monospace, SFMono-Regular, monospace; font-size: 13px; resize: none; color: var(--text-primary); }
  .modal-actions { display: flex; justify-content: flex-end; gap: 12px; }
  .outline-btn { display: flex; align-items: center; gap: 8px; padding: 10px 18px; border-radius: 14px; border: 1px solid var(--border-subtle); background: transparent; color: var(--text-primary); font-size: 14px; font-weight: 600; cursor: pointer; }
  .primary-btn { padding: 12px 28px; border-radius: 14px; background: #AF52DE; color: white; border: none; font-weight: 700; cursor: pointer; transition: 0.2s; }
  .primary-btn:hover { background: #9b40bd; transform: translateY(-1px); }
  .text-btn { background: transparent; border: none; color: var(--text-secondary); display: flex; align-items: center; gap: 6px; font-weight: 600; cursor: pointer; }

  .fade-in { animation: fadeIn 0.4s cubic-bezier(0.23, 1, 0.32, 1); }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(16px); } to { opacity: 1; transform: translateY(0); } }
</style>
