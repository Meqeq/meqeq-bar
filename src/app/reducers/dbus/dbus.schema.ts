export enum TrayItemStatus {
  Active = 'Active',
  Passive = 'Passive',
  NeedsAttention = 'NeedsAttention',
}

export interface RegisteredTrayItem {
  id: string;
  title: string;
  status: TrayItemStatus;
}

export interface TrayItemNewIcon {
  id: string;
  icon: number[];
}

export interface TrayItemNewProp {
  id: string;
  prop: string;
  propName: string;
}

export interface TrayItem {
  id: string;
  icon: string;
  title: string;
  status: TrayItemStatus;
  menu: MenuEntry[];
}

export enum MenuEntryType {
  Entry = 'entry',
  Separator = 'separator',
}

export interface MenuEntry {
  id: number;
  label: string;
  visible: boolean;
  type: MenuEntryType;
}

export interface TrayItemNewMenu {
  id: string;
  menu: MenuEntry[];
}
