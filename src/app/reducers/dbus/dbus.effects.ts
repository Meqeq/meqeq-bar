import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';

import { map, switchMap } from 'rxjs/operators';
import { DbusActions } from './dbus.actions';
import { fromTauriEvent, fromTauriEventString } from '../../common/tauri-utils';
import {
  RegisteredTrayItem,
  TrayItemNewIcon,
  TrayItemNewMenu,
  TrayItemNewProp,
} from './dbus.schema';
import { from } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';

@Injectable()
export class DbusEffects {
  private readonly actions$ = inject(Actions);

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

  readonly trayItemMenus$ = createEffect(() => {
    return fromTauriEvent<TrayItemNewMenu>('dbus_tray_item_new_menu').pipe(
      map((menu) => DbusActions.trayItemNewMenu(menu)),
    );
  });

  readonly calls$ = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(DbusActions.callTrayMenuOption),
        switchMap(({ itemId, entryId }) => {
          return from(
            invoke('dbus_tray_item_call_menu', {
              itemId,
              entryId,
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );
}
