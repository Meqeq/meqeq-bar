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
