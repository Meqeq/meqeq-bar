import { Component, effect, inject } from '@angular/core';

import { PopoverService } from '../../common/popover.service';
import { Store } from '@ngrx/store';
import { selectTrayItemsArray } from '../../reducers/dbus/dbus.selectors';

@Component({
  selector: 'app-tray',
  templateUrl: './tray.component.html',
  imports: [],
})
export class TrayComponent {
  private readonly store = inject(Store);
  readonly popoverService = inject(PopoverService);
  // readonly barService = inject(BarService);

  readonly items = this.store.selectSignal(selectTrayItemsArray);

  // callMenuItem(params: { service: string; path: string; id: number }): void {
  //   invoke('call_tray_menu_item', params);
  // }

  // constructor() {
  //   effect(() => {
  //     console.log(this.barService.trayItems());
  // dd
  //   });
  // }
}
