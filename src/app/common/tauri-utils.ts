import { Observable } from "rxjs";
import { Event } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export const fromTauriEvent = <Payload>(
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
