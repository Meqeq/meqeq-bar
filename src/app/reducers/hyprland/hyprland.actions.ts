import { createActionGroup, emptyProps, props } from '@ngrx/store';
import { ActiveWindow, WorkspaceInfo } from './hyprland.reducer';

export const HyprlandActions = createActionGroup({
  source: 'Hyprland',
  events: {
    Workspaces: props<{ workspaces: WorkspaceInfo[] }>(),
    'Active window': props<{ activeWindow: ActiveWindow }>(),
    'Active workspace': props<{ activeWorkspace: number }>(),
  },
});
