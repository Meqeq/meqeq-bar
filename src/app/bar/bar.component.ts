import { DatePipe } from "@angular/common";
import { Component } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { interval, map, startWith } from "rxjs";

@Component({
  standalone: true,
  selector: "app-bar",
  templateUrl: "./bar.component.html",
  imports: [DatePipe],
})
export class BarComponent {
  readonly time = toSignal(
    interval(1000).pipe(
      startWith(null),
      map(() => new Date()),
    ),
  );

  lel(): void {
    invoke("open_window").then(() => {
      console.log("DAWDAWD");
    });
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
