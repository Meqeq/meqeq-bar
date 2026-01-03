import { Injectable } from '@angular/core';
import { createEffect } from '@ngrx/effects';

import { map } from 'rxjs/operators';
import { DbusActions } from './dbus.actions';
import { fromTauriEvent, fromTauriEventString } from '../../common/tauri-utils';
import {
  RegisteredTrayItem,
  TrayItemNewIcon,
  TrayItemNewProp,
} from './dbus.schema';

@Injectable()
export class DbusEffects {
  readonly registeredTrayItems$ = createEffect(() => {
    return fromTauriEvent<RegisteredTrayItem>('dbus_register_tray_item').pipe(
      map((item) => DbusActions.registerTrayItem({ item })),
    );
  });

  readonly unregisteredTrayItems$ = createEffect(() => {
    return fromTauriEventString('dbus_unregister_tray_item').pipe(
      map((id) => DbusActions.unregisterTrayItem({ id })),
    );
  });

  readonly icons$ = createEffect(() => {
    return fromTauriEvent<TrayItemNewIcon>('dbus_tray_item_new_icon').pipe(
      map((item) => {
        const content = new Uint8Array(item.icon);

        return DbusActions.trayItemNewIcon({
          id: item.id,
          icon: URL.createObjectURL(
            new Blob([content.buffer], { type: 'image/png' } /* (1) */),
          ),
        });
      }),
    );
  });

  readonly trayItemProps$ = createEffect(() => {
    return fromTauriEvent<TrayItemNewProp>('dbus_tray_item_new_prop').pipe(
      map((prop) => DbusActions.trayItemNewProp(prop)),
    );
  });
}
