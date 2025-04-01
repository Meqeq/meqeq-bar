import { DatePipe } from "@angular/common";
import { Component } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { interval, map, startWith } from "rxjs";
import { PillComponent } from "../../common/pill/pill.component";

@Component({
  selector: "app-clock",
  templateUrl: "./clock.component.html",
  imports: [DatePipe, PillComponent],
})
export class ClockComponent {
  readonly time = toSignal(
    interval(1000).pipe(
      startWith(null),
      map(() => new Date()),
    ),
  );

  openCalendar(): void {
    invoke("open_popup", { popup: "calendar" }).then(() => {
      console.log("DAWDAWD");
    });
  }
}
