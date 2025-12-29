import { isDevMode } from '@angular/core';
import { ActionReducer, ActionReducerMap, MetaReducer } from '@ngrx/store';

import * as hyprland from './hyprland/hyprland.reducer';
import * as pipewire from './pipewire/pipewire.reducer';
import { routerReducer, RouterReducerState } from '@ngrx/router-store';

export const globalFridgeFeatureKey = 'globalFridge';

export interface State {
  [hyprland.hyprlandFeatureKey]: hyprland.State;
  [pipewire.pipewireFeatureKey]: pipewire.State;
  router: RouterReducerState;
}

export const reducers: ActionReducerMap<State> = {
  [hyprland.hyprlandFeatureKey]: hyprland.reducer,
  [pipewire.pipewireFeatureKey]: pipewire.reducer,
  router: routerReducer,
};

export function debug(reducer: ActionReducer<any>): ActionReducer<any> {
  return function (state, action) {
    console.log('state', state);
    console.log('action', action);

    if (action.type === '@ngrx/store/init') {
      const kek = localStorage.getItem('karwasz');
      if (kek) {
        return reducer(JSON.parse(kek), action);
      }
    }

    localStorage.setItem('karwasz', JSON.stringify(state));

    return reducer(state, action);
  };
}

export const metaReducers: MetaReducer<State>[] = isDevMode() ? [debug] : [];
