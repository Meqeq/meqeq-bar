import { Injectable } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { PwDevice, PwDeviceRoute } from './sound-new.schema';
import { fromTauriEvent } from '../common/tauri-utils';
import { scan, tap } from 'rxjs';

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
export class SoundNewService {
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
}
