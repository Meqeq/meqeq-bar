import { computed, Injectable } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import {
  PwDefault,
  PwDevice,
  PwDeviceProfile,
  PwDeviceRoute,
  PwNode,
  PwNodeProps,
} from './sound.schema';
import { fromTauriEvent } from '../common/tauri-utils';
import { map, merge, scan, tap } from 'rxjs';

interface EnumRoutes {
  input: Map<number, PwDeviceRoute>;
  output: Map<number, PwDeviceRoute>;
}

interface CurrentRoutes {
  input: PwDeviceRoute | undefined;
  output: PwDeviceRoute | undefined;
}

@Injectable({
  providedIn: 'root',
})
export class SoundService {
  readonly nodes = toSignal(
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
    { equal: () => false, initialValue: new Map<number, PwNode>() },
  );

  readonly nodesProps = toSignal(
    fromTauriEvent<PwNodeProps>('pw_node_props').pipe(
      scan((acc, props) => {
        acc.set(props.id, props);

        return acc;
      }, new Map<number, PwNodeProps>()),
    ),
    { equal: () => false, initialValue: new Map<number, PwNodeProps>() },
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

  readonly deviceEnumRoutes = toSignal(
    fromTauriEvent<PwDeviceRoute>('pw_device_enum_route').pipe(
      scan((acc, enumRoute) => {
        if (enumRoute.direction === 'unknown') return acc;

        let current = acc.get(enumRoute.deviceId);

        if (!current) {
          current = {
            input: new Map<number, PwDeviceRoute>(),
            output: new Map<number, PwDeviceRoute>(),
          };

          acc.set(enumRoute.deviceId, current);
        }

        current[enumRoute.direction].set(enumRoute.index, enumRoute);

        return acc;
      }, new Map<number, EnumRoutes>()),
    ),
    { equal: () => false, initialValue: new Map<number, EnumRoutes>() },
  );

  readonly deviceEnumProfiles = toSignal(
    fromTauriEvent<PwDeviceProfile>('pw_device_enum_profile').pipe(
      scan((acc, enumProfile) => {
        let current = acc.get(enumProfile.deviceId);

        if (!current) {
          current = new Map<number, PwDeviceProfile>();

          acc.set(enumProfile.deviceId, current);
        }

        current.set(enumProfile.index, enumProfile);

        return acc;
      }, new Map<number, Map<number, PwDeviceProfile>>()),
    ),
    {
      equal: () => false,
      initialValue: new Map<number, Map<number, PwDeviceProfile>>(),
    },
  );

  readonly deviceRoute = toSignal(
    fromTauriEvent<PwDeviceRoute>('pw_device_route').pipe(
      scan((acc, route) => {
        if (route.direction === 'unknown') return acc;

        const current = acc.get(route.deviceId) ?? {
          input: undefined,
          output: undefined,
        };

        acc.set(route.deviceId, {
          ...current,
          [route.direction]: route,
        });

        return acc;
      }, new Map<number, CurrentRoutes>()),
    ),
    { equal: () => false, initialValue: new Map<number, CurrentRoutes>() },
  );

  readonly deviceProfile = toSignal(
    fromTauriEvent<PwDeviceProfile>('pw_device_profile').pipe(
      scan((acc, route) => {
        acc.set(route.deviceId, route);

        return acc;
      }, new Map<number, PwDeviceProfile>()),
    ),
    { equal: () => false, initialValue: new Map<number, PwDeviceProfile>() },
  );

  readonly defaultSinkName = toSignal(
    fromTauriEvent<PwDefault>('pw_default_sink').pipe(
      map((res) => {
        console.log(res);

        return res.name;
      }),
    ),
  );

  readonly defaultSinkDevice = computed(() => {
    console.log(this.devices(), this.defaultSinkName());

    console.log(this.nodes());
    let defaultNode: PwNode | undefined;
    this.nodes().forEach((node) => {
      if (node.name === this.defaultSinkName()) defaultNode = node;
    });

    if (!defaultNode) return null;

    const defaultDevice = this.devices().get(defaultNode.deviceId);
    console.log(defaultDevice);
    return defaultDevice;
  });

  readonly defaultSourceName = toSignal(
    fromTauriEvent<PwDefault>('pw_default_source').pipe(
      map((res) => {
        return res.name;
      }),
    ),
  );

  readonly defaultSourceDevice = computed(() => {
    let defaultNode: PwNode | undefined;
    this.nodes().forEach((node) => {
      if (node.name === this.defaultSinkName()) defaultNode = node;
    });

    if (!defaultNode) return null;

    const defaultDevice = this.devices().get(defaultNode.deviceId);
    console.log(defaultDevice);
    return defaultDevice;
  });

  readonly defaultSource = toSignal(
    fromTauriEvent<PwDefault>('pw_default_source').pipe(map((res) => res.name)),
  );
}
