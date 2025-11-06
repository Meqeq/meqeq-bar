import { Component, effect, inject } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { invoke } from '@tauri-apps/api/core';
import { map, merge, Observable, scan } from 'rxjs';

import { fromTauriEvent } from '../../common/tauri-utils';
import { BarService } from '../bar.service';
import { PopoverService } from '../../common/popover.service';

@Component({
  selector: 'app-tray',
  templateUrl: './tray.component.html',
  imports: [],
})
export class TrayComponent {
  readonly popoverService = inject(PopoverService);
  readonly barService = inject(BarService);

  callMenuItem(params: { service: string; path: string; id: number }): void {
    invoke('call_tray_menu_item', params);
  }

  constructor() {
    effect(() => {
      console.log(this.barService.trayItems());
    });
  }
}
