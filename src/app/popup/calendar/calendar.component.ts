import { Component, effect, ElementRef, viewChild } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";

import * as Pikaday from "pikaday";

@Component({
  standalone: true,
  selector: "app-calendar",
  templateUrl: "./calendar.component.html",
})
export class CalendarComponent {
  readonly datePicker =
    viewChild.required<ElementRef<HTMLInputElement>>("datePicker");

  readonly calendar =
    viewChild.required<ElementRef<HTMLInputElement>>("calendar");

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
    setTimeout(() => {
      this.datePicker().nativeElement.click();
    });
  }

  close(): void {
    invoke("close_window").then(() => {
      console.log("DAWDAWD");
    });
  }
}
