import {
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
  provideZonelessChangeDetection,
} from '@angular/core';
import { provideRouter, withRouterConfig } from '@angular/router';

import { routes } from './app.routes';
import { provideEffects } from '@ngrx/effects';
import { provideStore } from '@ngrx/store';
import { metaReducers, reducers } from './reducers';
import { HyprlandEffects } from './reducers/hyprland/hyprland.effects';
import { provideRouterStore } from '@ngrx/router-store';
import { PipewireEffects } from './reducers/pipewire/pipewire.effects';
import { DbusEffects } from './reducers/dbus/dbus.effects';
import { BarEffects } from './reducers/bar/bar.effects';
import { PlayerEffects } from './reducers/player/player.effects';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideZonelessChangeDetection(),
    provideRouter(
      routes,
      withRouterConfig({ paramsInheritanceStrategy: 'always' }),
    ),
    provideEffects([
      HyprlandEffects,
      PipewireEffects,
      PlayerEffects,
      DbusEffects,
      BarEffects,
    ]),
    provideStore(reducers, {
      metaReducers,
    }),
    provideRouterStore(),
  ],
};
