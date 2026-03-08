import { isDevMode } from '@angular/core';
import { ActionReducer, ActionReducerMap, MetaReducer } from '@ngrx/store';

import * as hyprland from './hyprland/hyprland.reducer';
import * as pipewire from './pipewire/pipewire.reducer';
import * as dbus from './dbus/dbus.reducer';
import * as bar from './bar/bar.reducer';
import { routerReducer, RouterReducerState } from '@ngrx/router-store';

export const globalFridgeFeatureKey = 'globalFridge';

export interface State {
  [hyprland.hyprlandFeatureKey]: hyprland.State;
  [pipewire.pipewireFeatureKey]: pipewire.State;
  [dbus.dbusFeatureKey]: dbus.State;
  [bar.barFeatureKey]: bar.State;
  router: RouterReducerState;
}

export const reducers: ActionReducerMap<State> = {
  [hyprland.hyprlandFeatureKey]: hyprland.reducer,
  [pipewire.pipewireFeatureKey]: pipewire.reducer,
  [dbus.dbusFeatureKey]: dbus.reducer,
  [bar.barFeatureKey]: bar.reducer,
  router: routerReducer,
};

export function debug(reducer: ActionReducer<any>): ActionReducer<any> {
  return function (state, action) {
    // console.log('state', state);
    // console.log('action', action);

    // if (action.type === '@ngrx/store/init') {
    //   const kek = localStorage.getItem('karwasz');
    //   if (kek) {
    //     return reducer(JSON.parse(kek), action);
    //   }
    // }

    // localStorage.setItem('karwasz', JSON.stringify(state));

    return reducer(state, action);
  };
}

export const metaReducers: MetaReducer<State>[] = isDevMode() ? [debug] : [];
