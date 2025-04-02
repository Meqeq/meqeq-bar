import { DatePipe } from "@angular/common";
import { Component, effect, ElementRef, viewChild } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";

import * as Pikaday from "pikaday";
import { interval, map, startWith } from "rxjs";

@Component({
  standalone: true,
  selector: "app-calendar",
  templateUrl: "./calendar.component.html",
  imports: [DatePipe],
})
export class CalendarComponent {
  readonly datePicker =
    viewChild.required<ElementRef<HTMLInputElement>>("datePicker");

  readonly calendar =
    viewChild.required<ElementRef<HTMLInputElement>>("calendar");

  readonly time = toSignal(
    interval(1000).pipe(
      startWith(null),
      map(() => new Date()),
    ),
  );

  constructor() {
    // effect(() => {
    //   console.log(this.datePicker());
    //   new Pikaday.default({ field: this.datePicker().nativeElement });
    // });

    effect(() => {
      console.log("DAWDAWD");
      new Pikaday.default({
        field: this.datePicker().nativeElement,
        container: this.calendar().nativeElement,
        bound: false,
      });
    });
  }

  ngOnInit(): void {
    // setTimeout(() => {
    //   this.datePicker().nativeElement.click();
    // });
  }
}
