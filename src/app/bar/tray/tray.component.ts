import { JsonPipe } from "@angular/common";
import { Component, computed, effect, inject, signal } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { map, merge, Observable, scan, share } from "rxjs";
import { PillComponent } from "../../common/pill/pill.component";
import { PopoverService } from "../../common/popover.service";
import { fromTauriEvent } from "../../common/tauri-utils";
import { BarService } from "../bar.service";

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

export interface TrayItemMenu {
  service: string;
  entries: {
    position: number;
    label: string;
    visible: boolean;
    type: "separator" | "";
  }[];
}

@Component({
  selector: "app-tray",
  templateUrl: "./tray.component.html",
  imports: [PillComponent, JsonPipe],
})
export class TrayComponent {
  readonly popoverService = inject(PopoverService);
  readonly barService = inject(BarService);

  private readonly trayItemMenu$ = fromTauriEvent<TrayItemMenu>(
    "tray_item_menu",
  ).pipe(
    map((menu) => menu.payload),
    scan((menus, menu) => {
      menus.set(menu.service, menu);

      return menus;
    }, new Map<string, TrayItemMenu>()),
  );

  readonly menus = toSignal(this.trayItemMenu$, {
    equal: () => false,
    initialValue: new Map<string, TrayItemMenu>(),
  });

  readonly trayItems = toSignal(
    merge(this.getTrayItems("add"), this.getTrayItems("remove")).pipe(
      scan((items, item) => {
        if (item.type === "add") return [...items, item];
        else return items.filter((i) => i.service !== item.service);
      }, [] as TrayItem[]),
    ),
  );

  private getTrayItems<T extends "add" | "remove">(
    type: T,
  ): Observable<TrayItem & { type: T }> {
    return fromTauriEvent<TrayItemPayload>(`tray_item_${type}`).pipe(
      map((event) => {
        const content = new Uint8Array(event.payload.icon);

        return {
          type,
          title: event.payload.title,
          service: event.payload.service,
          icon: URL.createObjectURL(
            new Blob([content.buffer], { type: "image/png" } /* (1) */),
          ),
        };
      }),
    );
  }
}
