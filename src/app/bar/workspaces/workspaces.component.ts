import { Component, inject } from '@angular/core';
import { Store } from '@ngrx/store';
import {
  selectActiveWorkspace,
  selectWorkspaces,
} from '../../reducers/hyprland/hyprland.selectors';
import { HyprlandActions } from '../../reducers/hyprland/hyprland.actions';

@Component({
  selector: 'app-workspaces',
  templateUrl: './workspaces.component.html',
})
export class WorkspacesComponent {
  private readonly store = inject(Store);

  readonly workspaces = this.store.selectSignal(selectWorkspaces);
  readonly activeWorkspace = this.store.selectSignal(selectActiveWorkspace);

  setCurrentWorkspace(id: number): void {
    this.store.dispatch(HyprlandActions.setActiveWorkspace({ id }));
  }
}
