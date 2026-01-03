import { Component, inject } from '@angular/core';

import { PopoverService } from '../../common/popover.service';
import { Store } from '@ngrx/store';
import { selectTrayItemsArray } from '../../reducers/dbus/dbus.selectors';
import { DbusActions } from '../../reducers/dbus/dbus.actions';

@Component({
  selector: 'app-tray',
  templateUrl: './tray.component.html',
  imports: [],
})
export class TrayComponent {
  private readonly store = inject(Store);
  readonly popoverService = inject(PopoverService);

  readonly items = this.store.selectSignal(selectTrayItemsArray);

  callMenuItem(itemId: string, entryId: number): void {
    this.store.dispatch(
      DbusActions.callTrayMenuOption({
        itemId,
        entryId,
      }),
    );
  }
}
