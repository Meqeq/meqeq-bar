import { Component, signal } from "@angular/core";
import { CommonModule } from "@angular/common";
import { invoke } from "@tauri-apps/api/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { interval, map, startWith } from "rxjs";
import { Window } from "@tauri-apps/api/window";
import { Webview } from "@tauri-apps/api/webview";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { RouterOutlet } from "@angular/router";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent {
  greetingMessage = "";

  readonly time = toSignal(
    interval(1000).pipe(
      startWith(null),
      map(() => new Date()),
    ),
  );

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }

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
