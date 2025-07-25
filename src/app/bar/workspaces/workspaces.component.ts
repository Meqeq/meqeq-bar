import { Component, effect, inject } from '@angular/core';
import { fromTauriEvent } from '../../common/tauri-utils';
import { rxResource } from '@angular/core/rxjs-interop';
import { BarService } from '../bar.service';
import { map } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';

interface WorkspaceInfo {
  id: number;
  name: string;
  monitor: number;
}

@Component({
  selector: 'app-workspaces',
  templateUrl: './workspaces.component.html',
})
export class WorkspacesComponent {
  private readonly barService = inject(BarService);
  // readonly activeWorkspace =
  //   fromTauriEvent("active_workspace_change", 0);

  readonly activeWorkspace = rxResource({
    stream: () => fromTauriEvent<number>('active_workspace_change'),
  });

  readonly workspaces = rxResource({
    stream: () =>
      fromTauriEvent<WorkspaceInfo[]>('workspaces').pipe(
        map((workspaces) =>
          workspaces.filter((w) => w.monitor === this.barService.monitor()),
        ),
      ),
  });

  c = effect(() => {
    console.log(this.activeWorkspace.value());
    console.log(this.workspaces.value());
  });

  // readonly workspaces = fromTauriEvent("workspaces", [] as WorkspaceInfo[])

  // ngOnInit(): void { }

  setCurrentWorkspace(id: number): void {
    console.log(id);
    invoke('set_current_workspace', { id });
  }
}
