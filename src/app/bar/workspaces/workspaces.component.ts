import { Component, input } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { map } from "rxjs";
import { fromTauriEvent } from "../../common/tauri-utils";

interface WorkspaceInfo {
  id: number;
  name: string;
  monitor: number;
}

@Component({
  selector: "app-workspaces",
  templateUrl: "./workspaces.component.html",
  styleUrl: "./workspaces.component.scss",
})
export class WorkspacesComponent {
  readonly monitor = input.required<number>();

  readonly activeWorkspace = toSignal(
    fromTauriEvent<number>("active_workspace_change").pipe(
      map((event) => event.payload),
    ),
    { initialValue: 0 },
  );

  readonly workspaces = toSignal(
    fromTauriEvent<WorkspaceInfo[]>("workspaces").pipe(
      map((event) => event.payload),
      map((workspaces) =>
        workspaces.filter(
          (workspace: any) => workspace.monitor === this.monitor(),
        ),
      ),
      // tap(console.log),
    ),
    { initialValue: [] as WorkspaceInfo[] },
  );

  ngOnInit(): void { }

  setCurrentWorkspace(id: number): void {
    console.log(id);
    invoke("set_current_workspace", { id });
  }
}
