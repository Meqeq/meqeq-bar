import { inject, Injectable, signal } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { toSignal } from '@angular/core/rxjs-interop';
import { map, merge, Observable, scan } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';
import { fromTauriEvent } from '../common/tauri-utils';

export interface TrayMenuEntry {
  id: number;
  label: string;
  visible: boolean;
  type: 'separator' | '';
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

@Injectable()
export class BarService {
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);

  readonly monitor = toSignal(
    this.route.paramMap.pipe(
      map((paramMap) => Number.parseInt(paramMap.get('monitor') ?? '0')),
    ),
  );

  readonly isPopupOpen = signal(false);

  init() {
    invoke('initialize');
  }

  openPopup(event: MouseEvent, name: string): void {
    event.stopPropagation();

    invoke('set_layer', {
      layer: 'top',
      bar: this.monitor(),
    }).then(() => {
      this.isPopupOpen.set(true);
      this.router.navigate([name], { relativeTo: this.route });
    });
  }

  closePopup(): void {
    if (!this.isPopupOpen()) return;

    this.router.navigate(['./'], { relativeTo: this.route }).then(() => {
      this.isPopupOpen.set(false);
      invoke('set_layer', {
        layer: 'bottom',
        bar: this.monitor(),
      });
    });
  }

  readonly trayItems = toSignal(
    merge(this.getTrayItems('add'), this.getTrayItems('remove')).pipe(
      scan((items, item) => {
        if (item.type === 'add') return [...items, item];
        else return items.filter((i) => i.service !== item.service);
      }, [] as TrayItem[]),
    ),
    {
      initialValue: [],
    },
  );

  private getTrayItems<T extends 'add' | 'remove'>(
    type: T,
  ): Observable<TrayItem & { type: T }> {
    return fromTauriEvent<TrayItemPayload>(`tray_item_${type}`).pipe(
      map((event) => {
        const content = new Uint8Array(event.icon);
        console.log(event);
        return {
          type,
          menu: event.menu,
          path: event.path,
          title: event.title,
          service: event.service,
          menu_path: event.menu_path,
          icon: URL.createObjectURL(
            new Blob([content.buffer], { type: 'image/png' } /* (1) */),
          ),
        };
      }),
    );
  }
}
