export interface PwDevice {
  id: number;
  name: string;
  nick: string;
  description: string;
  mixerName: string;
  clientId: number;
  alsaName: string;
  cardName: string;
  iconName: string;
}

export interface PwDeviceRoute {
  index: number;
  name: string;
  deviceId: number;
  direction: 'input' | 'output' | 'unknown';
  description: string;
  available: boolean;
  profiles: number[];
  devices: number[];
  priority: number;
  volume: [number, number];
  mute: boolean;
}

export interface PwDeviceProfile {
  index: number;
  name: string;
  deviceId: number;
  description: string;
  priority: number;
  available: boolean;
  classes: string[];
}

export interface PwNode {
  id: number;
  type: string;
  nick: string;
  name: string;
  class: string;
  deviceId: number;
  description: string;
}

export interface PwNodeProps {
  id: number;
  volume: [number, number];
  muted: boolean;
}

export interface PwDefault {
  name: string;
}
