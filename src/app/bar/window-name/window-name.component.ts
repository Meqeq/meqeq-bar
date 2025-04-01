import { Component } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { map } from "rxjs";
import { PillComponent } from "../../common/pill/pill.component";
import { fromTauriEvent } from "../../common/tauri-utils";

@Component({
  selector: "app-window-name",
  templateUrl: "./window-name.component.html",
  imports: [PillComponent],
})
export class WindowNameComponent {
  readonly activeWindow = toSignal(
    fromTauriEvent<{ class: string; title: string }>(
      "active_window_change",
    ).pipe(map((event) => event.payload)),
    { initialValue: { class: "", title: "" } },
  );
}
