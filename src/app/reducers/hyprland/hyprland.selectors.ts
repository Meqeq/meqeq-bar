import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromHyprland from './hyprland.reducer';
import { selectRouteParam } from '../router/router.selectors';

export const selectHyprlandState = createFeatureSelector<fromHyprland.State>(
  fromHyprland.hyprlandFeatureKey,
);

export const selectWorkspaces = createSelector(
  selectHyprlandState,
  selectRouteParam('monitor'),
  (state, monitor) => {
    if (!monitor) return [];

    const parsed = Number.parseInt(monitor);
    return state.workspaces.filter((workspace) => workspace.monitor === parsed);
  },
);

export const selectActiveWindow = createSelector(
  selectHyprlandState,
  (state) => {
    return state.activeWindow?.title ?? '';
  },
);

export const selectActiveWorkspace = createSelector(
  selectHyprlandState,
  (state) => {
    return state.activeWorkspace;
  },
);
