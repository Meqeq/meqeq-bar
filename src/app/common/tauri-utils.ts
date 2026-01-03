import { Observable } from 'rxjs';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const appWebview = getCurrentWebviewWindow();

export const fromTauriEvent = <Payload>(
  eventName: string,
): Observable<Payload> => {
  return new Observable((subscriber) => {
    const unlisten = appWebview.listen(eventName, (event) => {
      subscriber.next(JSON.parse(event.payload as string) as Payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
};

export const fromTauriEventString = (eventName: string): Observable<string> => {
  return new Observable((subscriber) => {
    const unlisten = appWebview.listen(eventName, (event) => {
      subscriber.next(event.payload as string);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
};
