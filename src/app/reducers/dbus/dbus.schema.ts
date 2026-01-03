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
}
