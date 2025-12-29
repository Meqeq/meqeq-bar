import { Injectable } from '@angular/core';
import { createEffect } from '@ngrx/effects';

import { PipewireActions } from './pipewire.actions';
import { fromTauriEvent } from '../../common/tauri-utils';
import {
  PwNode,
  PwNodeProps,
  PwDevice,
  PwEnumRoutes,
  PwRouteDirection,
} from './pipewire.schema';
import { debounceTime, groupBy, map, mergeAll, scan } from 'rxjs';
import {
  PwDefault,
  PwDeviceProfile,
  PwDeviceRoute,
} from '../../sound/sound.schema';

@Injectable()
export class PipewireEffects {
  readonly addNode$ = createEffect(() => {
    return fromTauriEvent<PwNode>('pw_node').pipe(
      map((node) => PipewireActions.nodeAdded({ node })),
    );
  });

  readonly removeNode$ = createEffect(() => {
    return fromTauriEvent<number>('pw_node_removed').pipe(
      map((node) => PipewireActions.nodeRemoved({ node })),
    );
  });

  readonly nodeProps$ = createEffect(() => {
    return fromTauriEvent<PwNodeProps>('pw_node_props').pipe(
      groupBy((props) => props.id),
      map((props$) => props$.pipe(debounceTime(250))),
      mergeAll(),
      map((props) => PipewireActions.nodePropsSet({ props })),
    );
  });

  readonly device$ = createEffect(() => {
    return fromTauriEvent<PwDevice>('pw_device').pipe(
      map((device) => PipewireActions.deviceAdded({ device })),
    );
  });

  // readonly enumRoutes$ = createEffect(() => {
  //   return fromTauriEvent<PwDeviceRoute>('pw_device_enum_route').pipe(
  //     map((enumRoute) => PipewireActions.enumRouteAdded({ enumRoute })),
  //   );
  // });

  // readonly enumProfiles$ = createEffect(() => {
  //   return fromTauriEvent<PwDeviceProfile>('pw_device_enum_profile').pipe(
  //     map((enumProfile) => PipewireActions.enumProfileAdded({ enumProfile })),
  //   );
  // });

  readonly routes$ = createEffect(() => {
    return fromTauriEvent<PwDeviceRoute>('pw_device_route').pipe(
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
    return fromTauriEvent<PwDeviceProfile>('pw_device_profile').pipe(
      groupBy((profile) => profile.deviceId),
      map((profiles$) => profiles$.pipe(debounceTime(250))),
      mergeAll(),
      map((profile) => PipewireActions.profileAdded({ profile })),
    );
  });

  readonly defaultSink$ = createEffect(() => {
    return fromTauriEvent<PwDefault>('pw_default_sink').pipe(
      map((defaultSink) => PipewireActions.defaultSinkSet({ defaultSink })),
    );
  });

  readonly defaultSource$ = createEffect(() => {
    return fromTauriEvent<PwDefault>('pw_default_source').pipe(
      map((defaultSource) =>
        PipewireActions.defaultSourceSet({ defaultSource }),
      ),
    );
  });

  readonly enumProfiles$ = createEffect(() => {
    return fromTauriEvent<PwDeviceProfile>('pw_device_enum_profile').pipe(
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
    return fromTauriEvent<PwDeviceRoute>('pw_device_enum_route').pipe(
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

  constructor() {}
}
