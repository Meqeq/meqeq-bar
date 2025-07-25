import { Observable } from 'rxjs';
import { Event } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Signal } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';

export const fromTauriEvent = <Payload>(
  eventName: string,
): Observable<Payload> => {
  const appWebview = getCurrentWebviewWindow();

  return new Observable((subscriber) => {
    const unlisten = appWebview.listen(eventName, (event) => {
      subscriber.next(JSON.parse(event.payload as string) as Payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
};
