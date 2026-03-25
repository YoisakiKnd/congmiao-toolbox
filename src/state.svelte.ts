export type NavItem = {
  title: string;
  caption: string;
  icon: string;
};

export type ToolId =
  | 'timestamp'
  | 'json-format'
  | 'url-encode'
  | 'base64'
  | 'hash-check'
  | 'batch-rename'
  | 'sort-rule'
  | 'duplicate-scan';

export type ToolCommand = {
  id: ToolId;
  title: string;
  subtitle: string;
  shortcut: string;
  icon: string;
  accent: 'teal' | 'blue';
  keywords: string[];
};

export type ActivityEntry = {
  source: 'TEXT' | 'FILE' | 'SYSTEM';
  title: string;
  value: string;
  meta: string;
  accent: 'teal' | 'blue';
};

export type SettingsGroup = {
  title: string;
  items: string[];
};

export function formatMeta() {
  return new Date().toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });
}

export function formatDateTime(value: number) {
  const date = new Date(value);
  const year = date.getFullYear();
  const month = `${date.getMonth() + 1}`.padStart(2, '0');
  const day = `${date.getDate()}`.padStart(2, '0');
  const hour = `${date.getHours()}`.padStart(2, '0');
  const minute = `${date.getMinutes()}`.padStart(2, '0');
  const second = `${date.getSeconds()}`.padStart(2, '0');
  return `${year}-${month}-${day} ${hour}:${minute}:${second}`;
}

export const navItems: NavItem[] = [
  { title: '仪表盘', caption: '添加小组件 / 快捷跳转', icon: 'dashboard' },
  { title: '全部工具', caption: '工具集列表', icon: 'apps' },
  { title: '使用时长', caption: '应用屏幕使用时间', icon: 'schedule' },
];

export const commands: ToolCommand[] = [
  {
    id: 'timestamp',
    title: 'Timestamp Convert',
    subtitle: '复制当前 Unix 时间戳或处理拖拽文件',
    shortcut: 'Cmd/Ctrl+T',
    icon: 'schedule',
    accent: 'teal',
    keywords: ['timestamp', 'unix', 'time', 'convert', '日期', '时间戳'],
  },
  {
    id: 'json-format',
    title: 'JSON Format',
    subtitle: '读取剪贴板 JSON 并格式化复制',
    shortcut: 'Cmd/Ctrl+J',
    icon: 'data_object',
    accent: 'teal',
    keywords: ['json', 'format', 'pretty', 'clipboard', '格式化'],
  },
  {
    id: 'url-encode',
    title: 'URL Encode',
    subtitle: '编码剪贴板中的 URL 文本',
    shortcut: 'Cmd/Ctrl+U',
    icon: 'link',
    accent: 'teal',
    keywords: ['url', 'encode', 'query', 'uri'],
  },
  {
    id: 'base64',
    title: 'Base64',
    subtitle: '对剪贴板文本进行 Base64 编码',
    shortcut: 'Cmd/Ctrl+B',
    icon: 'encrypted',
    accent: 'teal',
    keywords: ['base64', 'encode', 'text'],
  },
  {
    id: 'hash-check',
    title: 'Hash Check',
    subtitle: '计算剪贴板文本的 SHA-256',
    shortcut: 'Cmd/Ctrl+H',
    icon: 'fingerprint',
    accent: 'blue',
    keywords: ['hash', 'sha', 'sha256', 'checksum'],
  },
  {
    id: 'batch-rename',
    title: 'Batch Rename',
    subtitle: '文件重命名工作台入口',
    shortcut: 'Cmd/Ctrl+R',
    icon: 'edit_square',
    accent: 'blue',
    keywords: ['rename', 'file', 'batch'],
  },
  {
    id: 'sort-rule',
    title: 'Sort by Rule',
    subtitle: '按规则整理文件的入口',
    shortcut: 'Cmd/Ctrl+S',
    icon: 'sort',
    accent: 'blue',
    keywords: ['sort', 'rule', 'file', 'organize'],
  },
  {
    id: 'duplicate-scan',
    title: 'Duplicate Scan',
    subtitle: '重复文件扫描入口',
    shortcut: 'Cmd/Ctrl+D',
    icon: 'content_copy',
    accent: 'blue',
    keywords: ['duplicate', 'scan', 'file'],
  },
];

export const settingsGroups: SettingsGroup[] = [
  { title: 'Stack', items: ['Svelte 5', 'Tailwind-free Custom UI', 'Tauri 2', 'Bun'] },
  { title: 'Automation', items: ['Cmd/Ctrl+K', 'Clipboard Watch', 'Drag to Run'] },
  { title: 'Window', items: ['Single Page', 'Collapsible Sidebar', 'No Scroll'] },
];

export class AppState {
  appVersion = $state('0.1.0');
  theme = $state<'dark' | 'light'>('light');
  sidebarCollapsed = $state(true);
  activeNavIndex = $state(0);
  activeToolId = $state<string | null>(null);
  settingsOpen = $state(false);
  commandOpen = $state(false);
  commandQuery = $state('');
  commandIndex = $state(0);

  currentUnix = $state(Math.floor(Date.now() / 1000));
  clipboardJsonReady = $state(false);
  clipboardJsonHint = $state('剪贴板暂未检测到 JSON');
  clipboardJsonPreview = $state('{ ... }');
  timestampDropActive = $state(false);
  timestampFlash = $state(false);
  jsonFlash = $state(false);
  activeToolPulse = $state<ToolId | null>(null);

  recentActivity = $state<ActivityEntry[]>([
    {
      source: 'SYSTEM',
      title: 'Workspace Ready',
      value: '等待零点击交互',
      meta: formatMeta(),
      accent: 'teal',
    },
    {
      source: 'TEXT',
      title: 'JSON Watch',
      value: '监听剪贴板内容',
      meta: formatMeta(),
      accent: 'teal',
    },
    {
      source: 'FILE',
      title: 'Drop to Run',
      value: '拖拽文件到 Timestamp',
      meta: formatMeta(),
      accent: 'blue',
    },
  ]);

  addActivity(entry: Omit<ActivityEntry, 'meta'>) {
    this.recentActivity = [{ ...entry, meta: formatMeta() }, ...this.recentActivity].slice(0, 4);
  }

  pulseTool(id: ToolId) {
    this.activeToolPulse = id;
    window.setTimeout(() => {
      if (this.activeToolPulse === id) {
        this.activeToolPulse = null;
      }
    }, 900);
  }

  flashTimestampTile() {
    this.timestampFlash = true;
    this.pulseTool('timestamp');
    window.setTimeout(() => {
      this.timestampFlash = false;
    }, 700);
  }

  flashJsonTile() {
    this.jsonFlash = true;
    this.pulseTool('json-format');
    window.setTimeout(() => {
      this.jsonFlash = false;
    }, 900);
  }

  toggleTheme() {
    this.theme = this.theme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', this.theme);
  }
}

export const appState = new AppState();
