import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';

import { concatMap, map, switchMap, withLatestFrom } from 'rxjs/operators';
import { Observable, EMPTY, from, of } from 'rxjs';
import { BarActions } from './bar.actions';
import { Store } from '@ngrx/store';
import { selectRouteParam } from '../router/router.selectors';
import { invoke } from '@tauri-apps/api/core';

import { fromTauriEvent, fromTauriEventString } from '../../common/tauri-utils';
import { Layer } from './bar.reducer';

@Injectable()
export class BarEffects {
  private readonly actions$ = inject(Actions);
  private readonly store = inject(Store);

  readonly layerSet$ = createEffect(() => {
    return fromTauriEventString('bar_set_layer').pipe(
      map((layer) => BarActions.layerSet({ layer: layer as Layer })),
    );
  });

  readonly setTopLayer = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(BarActions.setTopLayer),
        withLatestFrom(this.store.select(selectRouteParam('monitor'))),
        switchMap(([, monitor]) => {
          if (!monitor) return of(null);

          return from(
            invoke('set_layer', {
              layer: 'top',
              bar: Number.parseInt(monitor),
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );

  readonly setBottomLayer = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(BarActions.setBottomLayer),
        withLatestFrom(this.store.select(selectRouteParam('monitor'))),
        switchMap(([, monitor]) => {
          if (!monitor) return of(null);

          return from(
            invoke('set_layer', {
              layer: 'bottom',
              bar: Number.parseInt(monitor),
            }),
          );
        }),
      );
    },
    { dispatch: false },
  );
}
