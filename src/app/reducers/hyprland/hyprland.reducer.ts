import { createFeature, createReducer, on } from '@ngrx/store';
import { HyprlandActions } from './hyprland.actions';

export const hyprlandFeatureKey = 'hyprland';

export interface WorkspaceInfo {
  id: number;
  name: string;
  monitor: number;
}

export interface ActiveWindow {
  class: string;
  title: string;
}

export interface State {
  workspaces: WorkspaceInfo[];
  activeWorkspace: number;
  activeWindow: ActiveWindow | undefined;
}

export const initialState: State = {
  workspaces: [],
  activeWorkspace: 0,
  activeWindow: undefined,
};

export const reducer = createReducer(
  initialState,
  on(HyprlandActions.workspaces, (state, { workspaces }) => ({
    ...state,
    workspaces,
  })),
  on(HyprlandActions.activeWindow, (state, { activeWindow }) => ({
    ...state,
    activeWindow,
  })),
  on(HyprlandActions.activeWorkspace, (state, { activeWorkspace }) => ({
    ...state,
    activeWorkspace,
  })),
);

export const hyprlandFeature = createFeature({
  name: hyprlandFeatureKey,
  reducer,
});
