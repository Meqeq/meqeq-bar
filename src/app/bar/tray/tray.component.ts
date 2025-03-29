import { Component } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { fromTauriEvent } from "../../common/tauri-utils";
import { map, Observable, merge, scan, tap } from "rxjs";
import { JsonPipe, NgOptimizedImage } from "@angular/common";

export interface TrayItemPayload {
  service: string;
  title: string;
  icon: number[];
}

export interface TrayItem {
  service: string;
  title: string;
  icon: string;
}

@Component({
  selector: 'app-tray',
  templateUrl: './tray.component.html',
  imports: [NgOptimizedImage, JsonPipe]
})
export class TrayComponent {


  readonly trayItems = toSignal(

    merge(
      this.getTrayItems('add'),
      this.getTrayItems('remove'),
    ).pipe(
      scan((items, item) => {
        if (item.type === 'add')
          return [...items, item];
        else
          return items.filter(i => i.service !== item.service)

      }, [] as TrayItem[])
    )

  )

  private getTrayItems<T extends 'add' | 'remove'>(type: T): Observable<TrayItem & { type: T }> {
    return fromTauriEvent<TrayItemPayload>(`tray_item_${type}`).pipe(
      map(event => {

        const content = new Uint8Array(event.payload.icon);

        return {
          type,
          title: event.payload.title,
          service: event.payload.service,
          icon: URL.createObjectURL(
            new Blob([content.buffer], { type: 'image/png' } /* (1) */)
          )
        };

      })
    )
  }

}
