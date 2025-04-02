import { DatePipe, JsonPipe } from "@angular/common";
import { Component, inject } from "@angular/core";
import { SoundComponent } from "./sound/sound.component";

import { RouterOutlet } from "@angular/router";
import { BarService } from "./bar.service";
import { ClockComponent } from "./clock/clock.component";
import { TrayComponent } from "./tray/tray.component";
import { WindowNameComponent } from "./window-name/window-name.component";
import { WorkspacesComponent } from "./workspaces/workspaces.component";

@Component({
  selector: "app-bar",
  templateUrl: "./bar.component.html",
  providers: [BarService],
  imports: [
    DatePipe,
    RouterOutlet,
    SoundComponent,
    TrayComponent,
    ClockComponent,
    WindowNameComponent,
    WorkspacesComponent,
    JsonPipe,
  ],
})
export class BarComponent {
  readonly barService = inject(BarService);

  ngOnInit(): void {
    this.barService.init();
  }
}
