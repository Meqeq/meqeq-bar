import { createActionGroup, emptyProps, props } from '@ngrx/store';
import { RegisteredTrayItem, TrayItemNewProp } from './dbus.schema';

export const DbusActions = createActionGroup({
  source: 'Dbus',
  events: {
    'Register tray item': props<{ item: RegisteredTrayItem }>(),
    'Unregister tray item': props<{ id: string }>(),
    'Tray item new icon': props<{ id: string; icon: string }>(),
    'Tray item new prop': props<TrayItemNewProp>(),
  },
});
