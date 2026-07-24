import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';

import { PipewireActions } from './pipewire.actions';
import { fromTauriEvent } from '../../common/tauri-utils';
import {
  PwNode,
  PwNodeProps,
  PwDevice,
  PwEnumRoutes,
  PwRouteDirection,
  PwDeviceRoute,
  PwDeviceProfile,
  PwDefault,
} from './pipewire.schema';
import {
  debounceTime,
  from,
  groupBy,
  map,
  mergeAll,
  scan,
  switchMap,
  throwError,
  withLatestFrom,
} from 'rxjs';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@ngrx/store';
import { State } from './pipewire.reducer';
import { selectDevices, selectNodes } from './pipewire.selectors';

@Injectable()
export class PipewireEffects {
  private readonly actions$ = inject(Actions);
  private readonly store = inject(Store<State>);

  readonly addNode$ = createEffect(() => {
    return fromTauriEvent<PwNode>('Pipewire/Node').pipe(
      map((node) => PipewireActions.nodeAdded({ node })),
    );
  });

  readonly removeNode$ = createEffect(() => {
    return fromTauriEvent<number>('Pipewire/NodeRemoved').pipe(
      map((node) => PipewireActions.nodeRemoved({ node })),
    );
  });

  readonly nodeProps$ = createEffect(() => {
    return fromTauriEvent<PwNodeProps>('Pipewire/NodeProps').pipe(
      groupBy((props) => props.id),
      map((props$) => props$.pipe(debounceTime(250))),
      mergeAll(),
      map((props) => PipewireActions.nodePropsSet({ props })),
    );
  });

  readonly device$ = createEffect(() => {
    return fromTauriEvent<PwDevice>('Pipewire/Device').pipe(
      map((device) => PipewireActions.deviceAdded({ device })),
    );
  });

  readonly routes$ = createEffect(() => {
    return fromTauriEvent<PwDeviceRoute>('Pipewire/DeviceRoute').pipe(
      groupBy((route) => route.deviceId),
      map((routes$) =>
        routes$.pipe(
          groupBy((route) => route.direction),
          map((routes$) => routes$.pipe(debounceTime(250))),
          mergeAll(),
        ),
      ),
      mergeAll(),
      map((route) => PipewireActions.routeAdded({ route })),
    );
  });

  readonly profiles$ = createEffect(() => {
    return fromTauriEvent<PwDeviceProfile>('Pipewire/DeviceProfile').pipe(
      groupBy((profile) => profile.deviceId),
      map((profiles$) => profiles$.pipe(debounceTime(250))),
      mergeAll(),
      map((profile) => PipewireActions.profileAdded({ profile })),
    );
  });

  readonly defaultSink$ = createEffect(() => {
    return fromTauriEvent<PwDefault>('Pipewire/DefaultSink').pipe(
      map((defaultSink) => PipewireActions.defaultSinkSet({ defaultSink })),
    );
  });

  readonly defaultSource$ = createEffect(() => {
    return fromTauriEvent<PwDefault>('Pipewire/DefaultSource').pipe(
      map((defaultSource) =>
        PipewireActions.defaultSourceSet({ defaultSource }),
      ),
    );
  });

  readonly enumProfiles$ = createEffect(() => {
    return fromTauriEvent<PwDeviceProfile>('Pipewire/DeviceEnumProfile').pipe(
      groupBy((enumProfile) => enumProfile.deviceId),
      map((deviceProfiles$) =>
        deviceProfiles$.pipe(
          groupBy((enumProfile) => enumProfile.index),
          map((profiles$) => profiles$.pipe(debounceTime(250))),
          mergeAll(),
          scan(
            (acc, value) => {
              return { ...acc, [value.index]: value };
            },
            {} as Record<number, PwDeviceProfile>,
          ),
          debounceTime(10),
          map((enumProfiles) => ({
            deviceId: deviceProfiles$.key,
            enumProfiles,
          })),
        ),
      ),
      mergeAll(),
      map((result) => PipewireActions.enumProfilesAdded(result)),
    );
  });

  readonly enumRoutes$ = createEffect(() => {
    return fromTauriEvent<PwDeviceRoute>('Pipewire/DeviceEnumRoute').pipe(
      groupBy((enumProfile) => enumProfile.deviceId),
      map((deviceRoutes$) =>
        deviceRoutes$.pipe(
          groupBy((enumRoute) => enumRoute.index),
          map((routes$) => routes$.pipe(debounceTime(100))),
          mergeAll(),
          scan(
            (acc, route) => {
              return {
                ...acc,
                [route.direction]: {
                  ...acc[route.direction],
                  [route.index]: route,
                },
              };
            },
            {
              [PwRouteDirection.Input]: {},
              [PwRouteDirection.Output]: {},
            } as PwEnumRoutes,
          ),
          debounceTime(200),
          map((enumRoutes) => ({ deviceId: deviceRoutes$.key, enumRoutes })),
        ),
      ),
      mergeAll(),
      map((result) => PipewireActions.enumRoutesAdded(result)),
    );
  });

  readonly changeDeviceVolume$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.changeDeviceVolume),
        withLatestFrom(this.store.select(selectDevices)),
        switchMap(([{ id, volume, routeType }, devices]) => {
          const device = devices[id];
          if (!device) return throwError(() => new Error('Device not found'));

          const route = device.route[routeType];
          if (!route) return throwError(() => new Error('Route not found'));

          return from(
            invoke('set_device_volume', {
              id,
              routeIndex: route.index,
              routeDevice: route.devices[0],
              channelVolumes: [volume, volume],
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly muteDevice$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.muteDevice),
        withLatestFrom(this.store.select(selectDevices)),
        switchMap(([{ id, routeType }, devices]) => {
          const device = devices[id];
          if (!device) return throwError(() => new Error('Device not found'));

          const route = device.route[routeType];
          if (!route) return throwError(() => new Error('Route not found'));

          return from(
            invoke('set_device_mute', {
              id,
              routeIndex: route.index,
              routeDevice: route.devices[0],
              mute: true,
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly unmuteDevice$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.unmuteDevice),
        withLatestFrom(this.store.select(selectDevices)),
        switchMap(([{ id, routeType }, devices]) => {
          const device = devices[id];
          if (!device) return throwError(() => new Error('Device not found'));

          const route = device.route[routeType];
          if (!route) return throwError(() => new Error('Route not found'));

          return from(
            invoke('set_device_mute', {
              id,
              routeIndex: route.index,
              routeDevice: route.devices[0],
              mute: false,
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly setDefaultSink$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.setDefaultSink),
        withLatestFrom(this.store.select(selectNodes)),
        switchMap(([{ id }, nodes]) => {
          const node = Object.values(nodes).find(
            (node) => node.deviceId === id,
          );

          if (!node) return throwError(() => new Error('Node not found'));

          return from(
            invoke('set_default_sink', {
              sink: JSON.stringify({ name: node.name }),
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly setDefaultSource$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.setDefaultSource),
        withLatestFrom(this.store.select(selectNodes)),
        switchMap(([{ id }, nodes]) => {
          const node = Object.values(nodes).find(
            (node) => node.deviceId === id,
          );

          if (!node) return throwError(() => new Error('Node not found'));

          return from(
            invoke('set_default_source', {
              source: JSON.stringify({ name: node.name }),
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly setDeviceRoute$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.setDeviceRoute),
        withLatestFrom(this.store.select(selectDevices)),
        switchMap(([{ id, routeType, routeIndex }, devices]) => {
          const device = devices[id];
          if (!device) return throwError(() => new Error('Device not found'));

          const route = device.enumRoutes[routeType][routeIndex];
          if (!route) return throwError(() => new Error('Route not found'));

          return from(
            invoke('set_device_route', {
              id,
              routeIndex: route.index,
              routeDevice: route.devices[0],
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly setDeviceProfile$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.setDeviceProfile),
        switchMap(({ id, profileIndex }) => {
          return from(
            invoke('set_device_profile', {
              id,
              profileIndex,
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly changeNodeVolume$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.changeNodeVolume),
        switchMap(({ id, volume }) => {
          return from(
            invoke('set_node_volume', {
              id,
              channelVolumes: [volume, volume],
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly muteNode$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.muteNode),
        switchMap(({ id }) => {
          return from(
            invoke('set_node_mute', {
              id,
              mute: true,
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly unmuteNode$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PipewireActions.unmuteNode),
        switchMap(({ id }) => {
          return from(
            invoke('set_node_mute', {
              id,
              mute: false,
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );
}
