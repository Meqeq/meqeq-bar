import { DatePipe } from "@angular/common";
import { Component } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { interval, map, Observable, startWith } from "rxjs";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Event } from "@tauri-apps/api/event";

const fromTauriEvent = <Payload>(
  eventName: string,
): Observable<Event<Payload>> => {
  const appWebview = getCurrentWebviewWindow();

  return new Observable((subscriber) => {
    const unlisten = appWebview.listen(eventName, (event) => {
      subscriber.next({
        ...event,
        payload: JSON.parse(event.payload as string),
      } as Event<Payload>);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
};

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

  readonly activeWindow = toSignal(
    fromTauriEvent<{ class: string; title: string }>(
      "active_window_change",
    ).pipe(map((event) => event.payload)),
    { initialValue: { class: "", title: "" } },
  );

  ngOnInit(): void {
    // const appWebview = getCurrentWebviewWindow();
    // console.log("DAWDAWDAWD");

    // appWebview.listen<string>("active_window_change", (event) => {
    //   console.log(event);

    //   const payload = JSON.parse(event.payload);

    //   this.windowClass = payload.class;
    // });

    // fromTauriEvent<{ class: string, title: string }>("active_window_change").subscribe(console.log);

    invoke("active_window").then(() => {});
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
