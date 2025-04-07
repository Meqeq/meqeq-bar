import { Component, inject } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { invoke } from "@tauri-apps/api/core";
import { map, merge, Observable, scan } from "rxjs";
import { PillComponent } from "../../common/pill/pill.component";
import { PopoverService } from "../../common/popover.service";
import { fromTauriEvent } from "../../common/tauri-utils";
import { BarService } from "../bar.service";

export interface TrayMenuEntry {
  id: number;
  label: string;
  visible: boolean;
  type: "separator" | "";
}

export interface TrayItemPayload {
  service: string;
  path: string;
  title: string;
  icon: number[];
  menu: TrayMenuEntry[];
  menu_path: string;
}

export interface TrayItem {
  service: string;
  path: string;
  title: string;
  icon: string;
  menu: TrayMenuEntry[];
  menu_path: string;
}

@Component({
  selector: "app-tray",
  templateUrl: "./tray.component.html",
  imports: [PillComponent],
})
export class TrayComponent {
  readonly popoverService = inject(PopoverService);
  readonly barService = inject(BarService);

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
        console.log(event.payload);
        return {
          type,
          menu: event.payload.menu,
          path: event.payload.path,
          title: event.payload.title,
          service: event.payload.service,
          menu_path: event.payload.menu_path,
          icon: URL.createObjectURL(
            new Blob([content.buffer], { type: "image/png" } /* (1) */),
          ),
        };
      }),
    );
  }

  callMenuItem(params: { service: string; path: string; id: number }): void {
    invoke("call_tray_menu_item", params);
  }
}
