import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';

import { map, switchMap } from 'rxjs/operators';
import { HyprlandActions } from './hyprland.actions';
import { ActiveWindow, WorkspaceInfo } from './hyprland.reducer';
import { fromTauriEvent } from '../../common/tauri-utils';
import { from } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';

@Injectable()
export class HyprlandEffects {
  private readonly actions$ = inject(Actions);

  readonly workspaces$ = createEffect(() => {
    return fromTauriEvent<WorkspaceInfo[]>('Hyprland/Workspaces').pipe(
      map((workspaces) => HyprlandActions.workspaces({ workspaces })),
    );
  });

  readonly activeWindow$ = createEffect(() => {
    return fromTauriEvent<ActiveWindow>('Hyprland/ActiveWindow').pipe(
      map((activeWindow) => HyprlandActions.activeWindow({ activeWindow })),
    );
  });

  readonly activeWorkspace$ = createEffect(() => {
    return fromTauriEvent<number>('Hyprland/ActiveWorkspace').pipe(
      map((activeWorkspace) =>
        HyprlandActions.activeWorkspace({ activeWorkspace }),
      ),
    );
  });

  readonly setActiveWorkspace = createEffect(
    () => {
      return this.actions$.pipe(
        ofType(HyprlandActions.setActiveWorkspace),
        switchMap(({ id }) => {
          return from(invoke('set_current_workspace', { id }));
        }),
      );
    },
    { dispatch: false },
  );
}
