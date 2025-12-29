import { Injectable } from '@angular/core';
import { createEffect } from '@ngrx/effects';

import { map } from 'rxjs/operators';
import { HyprlandActions } from './hyprland.actions';
import { ActiveWindow, WorkspaceInfo } from './hyprland.reducer';
import { fromTauriEvent } from '../../common/tauri-utils';

@Injectable()
export class HyprlandEffects {
  readonly workspaces$ = createEffect(() => {
    return fromTauriEvent<WorkspaceInfo[]>('workspaces').pipe(
      map((workspaces) => HyprlandActions.workspaces({ workspaces })),
    );
  });

  readonly activeWindow$ = createEffect(() => {
    return fromTauriEvent<ActiveWindow>('active_window_change').pipe(
      map((activeWindow) => HyprlandActions.activeWindow({ activeWindow })),
    );
  });

  readonly activeWorkspace$ = createEffect(() => {
    return fromTauriEvent<number>('active_workspace_change').pipe(
      map((activeWorkspace) =>
        HyprlandActions.activeWorkspace({ activeWorkspace }),
      ),
    );
  });

  constructor() {}
}
