import { DatePipe, JsonPipe } from "@angular/common";
import { Component, input } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { interval, map, startWith, tap } from "rxjs";
import { SoundComponent } from "./sound/sound.component";

import { fromTauriEvent } from "../common/tauri-utils";
import { ClockComponent } from "./clock/clock.component";
import { TrayComponent } from "./tray/tray.component";
import { WindowNameComponent } from "./window-name/window-name.component";
import { WorkspacesComponent } from "./workspaces/workspaces.component";

@Component({
  selector: "app-bar",
  templateUrl: "./bar.component.html",
  imports: [
    DatePipe,
    SoundComponent,
    TrayComponent,
    ClockComponent,
    WindowNameComponent,
    WorkspacesComponent,
    JsonPipe,
  ],
})
export class BarComponent {
  readonly monitor = input(0, {
    transform: (v: any) => Number.parseInt(v),
  });
}
