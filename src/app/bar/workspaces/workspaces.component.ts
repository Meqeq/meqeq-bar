import { Component, inject } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@ngrx/store';
import {
  selectActiveWorkspace,
  selectWorkspaces,
} from '../../reducers/hyprland/hyprland.selectors';

@Component({
  selector: 'app-workspaces',
  templateUrl: './workspaces.component.html',
})
export class WorkspacesComponent {
  private readonly store = inject(Store);

  readonly workspaces = this.store.selectSignal(selectWorkspaces);
  readonly activeWorkspace = this.store.selectSignal(selectActiveWorkspace);

  setCurrentWorkspace(id: number): void {
    invoke('set_current_workspace', { id });
  }
}
