import { createFeature, createReducer, on } from '@ngrx/store';
import { DbusActions } from './dbus.actions';
import { TrayItem } from './dbus.schema';

export const dbusFeatureKey = 'dbus';

export interface State {
  trayItems: Record<string, TrayItem>;
}

export const initialState: State = {
  trayItems: {},
};

export const reducer = createReducer(
  initialState,
  on(DbusActions.registerTrayItem, (state, { item }) => ({
    ...state,
    trayItems: {
      ...state.trayItems,
      [item.id]: { ...item, icon: '' },
    },
  })),
  on(DbusActions.unregisterTrayItem, (state, { id }) => {
    const { [id]: ignored, ...trayItems } = state.trayItems;
    console.log('DDDUDUDUDUDUD');
    return {
      ...state,
      trayItems,
    };
  }),
  on(DbusActions.trayItemNewIcon, (state, { id, icon }) => {
    const item = state.trayItems[id];

    if (!item) return state;

    return {
      ...state,
      trayItems: {
        ...state.trayItems,
        [item.id]: {
          ...item,
          icon,
        },
      },
    };
  }),
  on(DbusActions.trayItemNewProp, (state, { id, prop, propName }) => {
    const item = state.trayItems[id];

    if (!item) return state;

    return {
      ...state,
      trayItems: {
        ...state.trayItems,
        [item.id]: {
          ...item,
          [propName]: prop,
        },
      },
    };
  }),
);

export const dbusFeature = createFeature({
  name: dbusFeatureKey,
  reducer,
});
