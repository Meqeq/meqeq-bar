import { Component, effect, inject } from '@angular/core';

import { PopoverService } from '../../common/popover.service';
import { Store } from '@ngrx/store';
import { selectTrayItemsArray } from '../../reducers/dbus/dbus.selectors';
import { DbusActions } from '../../reducers/dbus/dbus.actions';
import { BarActions } from '../../reducers/bar/bar.actions';
import { selectLayer } from '../../reducers/bar/bar.selectors';

@Component({
  selector: 'app-tray',
  templateUrl: './tray.component.html',
  imports: [],
})
export class TrayComponent {
  private readonly store = inject(Store);
  readonly popoverService = inject(PopoverService);

  readonly items = this.store.selectSignal(selectTrayItemsArray);
  readonly layer = this.store.selectSignal(selectLayer);

  callMenuItem(itemId: string, entryId: number): void {
    this.store.dispatch(
      DbusActions.callTrayMenuOption({
        itemId,
        entryId,
      }),
    );
  }

  ekek = effect(() => {
    console.log(this.items());
  });

  kek(event: ToggleEvent): void {
    console.log('DUDUDUDUDUUDUD', event);
    // event.preventDefault();

    if (event.newState === 'open')
      this.store.dispatch(BarActions.setTopLayer());
    else {
      setTimeout(() => {
        this.store.dispatch(BarActions.setBottomLayer());
      }, 50);
    }

    // setTimeout(() => {
    //   (event.target as HTMLElement).showPopover();
    //   // event.target?.dispatchEvent('');
    // }, 1000);
  }
}
