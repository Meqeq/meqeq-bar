export enum PwNodeClass {
  Sink = 'Audio/Sink',
  Source = 'Audio/Source',
  Playback = 'Stream/Output/Audio',
  Recording = 'Stream/Input/Audio',
  Midi = 'Midi/Bridge',
}

export enum PwRouteDirection {
  Input = 'input',
  Output = 'output',
  Unknown = 'unknown',
}

export interface PwNode {
  id: number;
  type: string;
  nick: string;
  name: string;
  class: PwNodeClass;
  deviceId: number;
  description: string;
}

export interface PwNodeProps {
  id: number;
  volume: [number, number];
  muted: boolean;
}

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

export type PwEnumRoutes = Record<
  PwRouteDirection,
  Record<number, PwDeviceRoute>
>;

export interface PwDeviceExtended extends PwDevice {
  route: {
    input: PwDeviceRoute | undefined;
    output: PwDeviceRoute | undefined;
  };
  enumRoutes: PwEnumRoutes;
  profile: PwDeviceProfile | undefined;
  enumProfiles: Record<number, PwDeviceProfile>;
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

export interface PwDefault {
  name: string;
}
