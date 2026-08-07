import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';

import { map, switchMap, tap } from 'rxjs/operators';
import { PlayerActions } from './player.actions';
import { fromTauriEvent } from '../../common/tauri-utils';
import { from } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';
import {
  LoopStatus,
  MediaPlayerResponse,
  MetadataMap,
  PlaybackStatus,
  PlayerAbility,
} from './player.schema';

@Injectable()
export class PlayerEffects {
  private readonly actions$ = inject(Actions);

  readonly newPlayer$ = createEffect(() => {
    return fromTauriEvent<MediaPlayerResponse>('Player/New').pipe(
      map((player) => PlayerActions.new({ player })),
    );
  });

  readonly disconnect$ = createEffect(() => {
    return fromTauriEvent<string>('Player/Disconnect').pipe(
      map((name) => PlayerActions.disconnect({ name })),
    );
  });

  readonly playbackState$ = createEffect(() => {
    return fromTauriEvent<[string, PlaybackStatus]>(
      'Player/PlaybackStatus',
    ).pipe(
      map(([name, status]) => PlayerActions.playbackStatus({ name, status })),
    );
  });

  readonly loopStatus$ = createEffect(() => {
    return fromTauriEvent<[string, LoopStatus]>('Player/LoopStatus').pipe(
      map(([name, status]) =>
        PlayerActions.loopStatusUpdated({ name, status }),
      ),
    );
  });

  readonly shuffle$ = createEffect(() => {
    return fromTauriEvent<[string, boolean]>('Player/Shuffle').pipe(
      map(([name, property]) =>
        PlayerActions.shuffleUpdated({ name, property }),
      ),
    );
  });

  readonly canGoNext$ = createEffect(() => {
    return fromTauriEvent<[string, PlayerAbility, boolean]>(
      'Player/Ability',
    ).pipe(
      map(([name, ability, value]) =>
        PlayerActions.abilityUpdated({ name, ability, value }),
      ),
    );
  });

  readonly metadata$ = createEffect(() => {
    return fromTauriEvent<[string, MetadataMap]>('Player/Metadata').pipe(
      map(([name, metadata]) => PlayerActions.metadata({ name, metadata })),
    );
  });

  readonly position$ = createEffect(() => {
    return fromTauriEvent<[string, number]>('Player/Position').pipe(
      map(([name, position]) =>
        PlayerActions.positionUpdated({ name, position }),
      ),
    );
  });

  readonly play = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PlayerActions.play),
        switchMap(({ name }) => {
          console.log(name);
          return from(invoke('player_play', { name }));
        }),
      );
    },
    { dispatch: false },
  );

  readonly pause = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PlayerActions.pause),
        switchMap(({ name }) => {
          console.log(name);
          return from(invoke('player_pause', { name }));
        }),
      );
    },
    { dispatch: false },
  );

  readonly next = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PlayerActions.next),
        switchMap(({ name }) => from(invoke('player_next', { name }))),
      );
    },
    { dispatch: false },
  );

  readonly prev = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PlayerActions.prev),
        switchMap(({ name }) => from(invoke('player_prev', { name }))),
      );
    },
    { dispatch: false },
  );

  readonly seek = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PlayerActions.seek),
        switchMap(({ name, position }) => {
          console.log(name);
          return from(invoke('player_seek', { name, position }));
        }),
      );
    },
    { dispatch: false },
  );

  readonly shuffle = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(PlayerActions.shuffle),
        switchMap(({ name, shuffle }) => {
          return from(invoke('player_shuffle', { name, shuffle }));
        }),
      );
    },
    { dispatch: false },
  );
}
