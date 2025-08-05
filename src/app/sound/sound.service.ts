import { computed, Injectable } from '@angular/core';
import { rxResource, toSignal } from '@angular/core/rxjs-interop';
import { merge, scan, startWith, tap } from 'rxjs';
import { fromTauriEvent } from '../common/tauri-utils';

export interface PipeWireMetadata {
  id: number;
  type: string;
  defaultSink: string;
  defaultSource: string;
}

export interface PipeWireNode {
  id: number;
  type: string;
  class: string;
  nick: string;
  description: string;
  name: string;
  muted: boolean;
  volume: number;
}

export interface PwNode {
  id: number;
  type: string;
  nick: string;
  name: string;
  class: string;
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
  alsaName: string;
  cardName: string;
  mixerName: string;
  iconName: string;
  clientId: number;
}

export interface PwDeviceProfile {
  deviceId: number;
  index: number;
  name: string;
  description: string;
  priority: number;
  available: boolean;
  classes: unknown[];
}

export interface PwDeviceRoute {
  deviceId: number;
  index: number;
  direction: 'input' | 'output' | 'unknown';
  name: string;
  description: string;
  priority: number;
  available: boolean;
  profiles: number[];
  devices: number[];
  volume: [number, number];
  mute: boolean;
}

@Injectable({
  providedIn: 'root',
})
export class SoundService {
  readonly defaults = rxResource({
    stream: () => fromTauriEvent<PipeWireMetadata>('pipewire_metadata'),
  });

  readonly nodespw = toSignal(
    merge(
      fromTauriEvent<PwNode>('pw_node'),
      fromTauriEvent<number>('pw_node_removed'),
    ).pipe(
      scan((acc, node) => {
        // console.log('NODE', node);
        if (typeof node === 'number') acc.delete(node);
        else acc.set(node.id, node);

        return acc;
      }, new Map<number, PwNode>()),
    ),
    { equal: () => false },
  );

  readonly nodespwprops = toSignal(
    fromTauriEvent<PwNodeProps>('pw_node_props').pipe(
      scan((acc, props) => {
        // console.log('NODE_PROPS', props);
        acc.set(props.id, props);

        return acc;
      }, new Map<number, PwNodeProps>()),
    ),
    { equal: () => false },
  );

  readonly defaultSinkName = toSignal(
    fromTauriEvent<{ name: string }>('pw_default_sink'),
    { initialValue: { name: '' } },
  );

  readonly defaultSourceName = toSignal(
    fromTauriEvent<{ name: string }>('pw_default_source'),
    { initialValue: { name: '' } },
  );

  readonly devices = toSignal(
    fromTauriEvent<PwDevice>('pw_device').pipe(
      scan((acc, device) => {
        acc.set(device.id, device);

        return acc;
      }, new Map<number, PwDevice>()),
    ),
    { equal: () => false, initialValue: new Map<number, PwDevice>() },
  );

  readonly deviceEnumProfiles = toSignal(
    fromTauriEvent<PwDeviceProfile>('pw_device_enum_profile').pipe(
      scan((acc, profile) => {
        if (acc.has(profile.deviceId)) {
          acc.get(profile.deviceId)?.set(profile.index, profile);
        } else {
          const map = new Map<number, PwDeviceProfile>();
          map.set(profile.index, profile);
          acc.set(profile.deviceId, map);
        }

        return acc;
      }, new Map<number, Map<number, PwDeviceProfile>>()),
    ),
    {
      equal: () => false,
      initialValue: new Map<number, Map<number, PwDeviceProfile>>(),
    },
  );

  readonly deviceEnumRoutes = toSignal(
    fromTauriEvent<PwDeviceRoute>('pw_device_enum_route').pipe(
      scan((acc, route) => {
        console.log('ENUM ROUTE', route);
        if (acc.has(route.deviceId)) {
          acc.get(route.deviceId)?.set(route.index, route);
        } else {
          const map = new Map<number, PwDeviceRoute>();
          map.set(route.index, route);
          acc.set(route.deviceId, map);
        }

        return acc;
      }, new Map<number, Map<number, PwDeviceRoute>>()),
    ),
    {
      equal: () => false,
      initialValue: new Map<number, Map<number, PwDeviceRoute>>(),
    },
  );

  readonly deviceProfiles = toSignal(
    fromTauriEvent<PwDeviceProfile>('pw_device_profile').pipe(
      scan((acc, profile) => {
        console.log('PROFILE', profile);
        if (acc.has(profile.deviceId)) {
          acc.get(profile.deviceId)?.set(profile.index, profile);
        } else {
          const map = new Map<number, PwDeviceProfile>();
          map.set(profile.index, profile);
          acc.set(profile.deviceId, map);
        }

        return acc;
      }, new Map<number, Map<number, PwDeviceProfile>>()),
    ),
    {
      equal: () => false,
      initialValue: new Map<number, Map<number, PwDeviceProfile>>(),
    },
  );

  readonly deviceRoutes = toSignal(
    fromTauriEvent<PwDeviceRoute>('pw_device_route').pipe(
      scan((acc, route) => {
        console.log('ROUTE', route);
        if (acc.has(route.deviceId)) {
          acc.get(route.deviceId)?.set(route.index, route);
        } else {
          const map = new Map<number, PwDeviceRoute>();
          map.set(route.index, route);
          acc.set(route.deviceId, map);
        }

        return acc;
      }, new Map<number, Map<number, PwDeviceRoute>>()),
    ),
    {
      equal: () => false,
      initialValue: new Map<number, Map<number, PwDeviceRoute>>(),
    },
  );

  readonly nodes = toSignal(
    fromTauriEvent<PipeWireNode>('pipewire_node').pipe(
      scan(
        (acc, node) => {
          // console.log(node);
          // console.log(node);

          if (acc.nodeMapId.has(node.id)) {
            acc.nodeMapId.set(node.id, node);
            acc.nodeMapName.set(node.name, node);

            return {
              ...acc,
              nodes: acc.nodes.map((n) => (n.id === node.id ? node : n)),
            };
          } else {
            acc.nodeMapId.set(node.id, node);
            acc.nodeMapName.set(node.name, node);

            return {
              ...acc,
              nodes: [...acc.nodes, node],
            };
          }
        },
        {
          nodes: [] as PipeWireNode[],
          nodeMapId: new Map<number, PipeWireNode>(),
          nodeMapName: new Map<string, PipeWireNode>(),
        },
      ),
      tap((kek) => {
        localStorage.setItem(
          'kek',
          JSON.stringify({
            ...kek,
            nodeMapId: [...kek.nodeMapId.entries()],
            nodeMapName: [...kek.nodeMapName.entries()],
          }),
        );
      }),
      startWith(this.getLocal()),
    ),
  );

  private getLocal(): any {
    const v = localStorage.getItem('kek');

    if (!v)
      return {
        nodes: [] as PipeWireNode[],
        nodeMapId: new Map<number, PipeWireNode>(),
        nodeMapName: new Map<string, PipeWireNode>(),
      };

    const p = JSON.parse(v);

    return {
      ...p,
      nodeMapId: new Map(p.nodeMapId),
      nodeMapName: new Map(p.nodeMapName),
    };
  }

  readonly defaultSink = computed(() => {
    const defaultName = this.defaults.value()?.defaultSink ?? '';
    const sink = this.nodes()?.nodeMapName.get(defaultName);
    return sink;
  });

  readonly defaultSource = computed(() => {
    const defaultName = this.defaults.value()?.defaultSource ?? '';
    const source = this.nodes()?.nodeMapName.get(defaultName);
    return source;
  });

  readonly defaultVolume = computed(() => {
    return (this.defaultSink()?.volume ?? 0) * 100;
  });
}
