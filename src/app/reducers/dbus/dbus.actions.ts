import { createActionGroup, props } from '@ngrx/store';
import {
  RegisteredTrayItem,
  TrayItemNewMenu,
  TrayItemNewProp,
} from './dbus.schema';

export const DbusActions = createActionGroup({
  source: 'Dbus',
  events: {
    'Register tray item': props<{ item: RegisteredTrayItem }>(),
    'Unregister tray item': props<{ id: string }>(),
    'Tray item new icon': props<{ id: string; icon: string }>(),
    'Tray item new prop': props<TrayItemNewProp>(),
    'Tray item new menu': props<TrayItemNewMenu>(),

    'Call tray menu option': props<{ itemId: string; entryId: number }>(),
  },
});
