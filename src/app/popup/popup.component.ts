import { Component, inject, input } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { ActivatedRoute } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { SoundPopupComponent } from "./sound/sound-popup.component";
import { fromTauriEvent } from "../common/tauri-utils";

import { PopupService } from "./popup.service";
import { JsonPipe } from "@angular/common";
import { CalendarComponent } from "./calendar/calendar.component";

@Component({
  standalone: true,
  selector: "app-popup",
  templateUrl: "./popup.component.html",
  imports: [SoundPopupComponent, CalendarComponent, JsonPipe],
})
export class PopupComponent {
  private readonly route = inject(ActivatedRoute);
  readonly service = inject(PopupService);

  readonly monitor = input.required<string>();

  ngOnInit(): void {
    console.log(this.monitor(), new Date().toISOString());
  }

  close(): void {
    invoke("close_popup").then(() => {
      console.log("DAWDAWD");
    });
  }
}
