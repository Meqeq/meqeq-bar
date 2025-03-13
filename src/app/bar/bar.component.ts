import { DatePipe } from "@angular/common";
import { Component, input } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { interval, map, startWith, tap } from "rxjs";
import { SoundComponent } from "./sound/sound.component";

import { fromTauriEvent } from "../common/tauri-utils";

interface WorkspaceInfo {
  id: number;
  name: string;
  monitor: number;
}

@Component({
  selector: "app-bar",
  templateUrl: "./bar.component.html",
  imports: [DatePipe, SoundComponent],
})
export class BarComponent {
  readonly monitor = input(0, {
    transform: (v: any) => Number.parseInt(v),
  });

  readonly time = toSignal(
    interval(1000).pipe(
      startWith(null),
      map(() => new Date()),
    ),
  );

  readonly activeWindow = toSignal(
    fromTauriEvent<{ class: string; title: string }>(
      "active_window_change",
    ).pipe(map((event) => event.payload)),
    { initialValue: { class: "", title: "" } },
  );

  readonly activeWorkspace = toSignal(
    fromTauriEvent<number>("active_workspace_change").pipe(
      map((event) => event.payload),
    ),
    { initialValue: 0 },
  );

  readonly workspaces = toSignal(
    fromTauriEvent<WorkspaceInfo[]>("workspaces").pipe(
      tap(console.log),
      map((event) => event.payload),
      map((workspaces) =>
        workspaces.filter(
          (workspace: any) => workspace.monitor === this.monitor(),
        ),
      ),
      tap(console.log),
    ),
    { initialValue: [] as WorkspaceInfo[] },
  );

  ngOnInit(): void {
    console.log("ADWDWD");

    // const appWebview = getCurrentWebviewWindow();
    // console.log("DAWDAWDAWD");
    // appWebview.listen<string>("active_window_change", (event) => {
    //   console.log(event);
    //   const payload = JSON.parse(event.payload);
    //   this.windowClass = payload.class;
    // });
    // fromTauriEvent<{ class: string, title: string }>("active_window_change").subscribe(console.log);
    // invoke("active_window").then(() => {});
    // invoke("on_add_workspace").then(() => {});
  }

  setCurrentWorkspace(id: number): void {
    console.log(id);
    invoke("set_current_workspace", { id });
  }

  openCalendar(): void {
    invoke("open_popup", { popup: "calendar" }).then(() => {
      console.log("DAWDAWD");
    });
  }

  lel(): void {
    // invoke("open_window").then(() => {
    //   console.log("DAWDAWD");
    // });
    // invoke("open_window2").then(() => {
    //   console.log("DAWDAWD");
    // });
    // loading embedded asset:
    // const webview = new WebviewWindow("theUniqueLabel", {
    //   url: "index.html",
    //   width: 400,
    //   height: 200,
    //   x: 30,
    //   y: 30,
    //   "title"
    // });
    // webview.once("tauri://created", function () {
    //   // webview successfully created
    //   console.log("AA");
    // });
    // webview.once("tauri://error", function (e) {
    //   // an error happened creating the webview
    //   console.log("BB", e);
    // });
  }
}
