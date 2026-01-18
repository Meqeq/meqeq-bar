import { Component, inject } from '@angular/core';
import { Store } from '@ngrx/store';
import { LogOut, LucideAngularModule, Power, RotateCcw } from 'lucide-angular';
import { BarActions } from '../../reducers/bar/bar.actions';

@Component({
  selector: 'app-power-menu-modal',
  templateUrl: './power-menu.component.html',
  imports: [LucideAngularModule],
})
export class PowerMenuModalComponent {
  private readonly store = inject(Store);

  readonly options = [
    {
      label: $localize`Wyloguj`,
      class: 'hover:text-warning',
      action: () => {
        this.store.dispatch(BarActions.logout());
      },
      icon: LogOut,
    },
    {
      label: $localize`Uruchom ponownie`,
      class: 'hover:text-success',
      action: () => {
        this.store.dispatch(BarActions.restart());
      },
      icon: RotateCcw,
    },
    {
      label: $localize`Wyłącz`,
      class: 'hover:text-error',
      action: () => {
        this.store.dispatch(BarActions.poweroff());
      },
      icon: Power,
    },
  ];
}
