import { Component, inject } from '@angular/core';
import { BarService } from './bar.service';
import { WorkspacesComponent } from './workspaces/workspaces.component';
import { StartComponent } from './start/start.component';
import { ClockComponent } from './clock/clock.component';
import { PowerMenuComponent } from './power-menu/power-menu.component';
import { RouterOutlet } from '@angular/router';
import { SoundComponent } from './sound/sound.component';
import { EthernetComponent } from './ethernet/ethernet.component';
import { TrayComponent } from './tray/tray.component';
import { Store } from '@ngrx/store';
import { selectActiveWindow } from '../reducers/hyprland/hyprland.selectors';
import { selectTrayHasItems } from '../reducers/dbus/dbus.selectors';
import { ModalPosition, ModalService } from '../common/modal.service';
import { SoundModalComponent } from '../modals/sound/sound.component';
import { CalendarModalComponent } from '../modals/calendar/calendar.component';
import { PowerMenuModalComponent } from '../modals/power-menu/power-menu.component';

@Component({
  selector: 'app-bar',
  templateUrl: './bar.component.html',
  imports: [
    WorkspacesComponent,
    StartComponent,
    SoundComponent,
    ClockComponent,
    TrayComponent,
    EthernetComponent,
    PowerMenuComponent,
  ],
  providers: [BarService],
  // host: {
  //   class: 'absolute z-1000000 block left-0 bottom-0 w-full',
  // },
  host: {
    class: 'block relative z-1000000',
  },
})
export class BarComponent {
  readonly barService = inject(BarService);
  private readonly store = inject(Store);
  private readonly modalService = inject(ModalService);

  readonly activeWindow = this.store.selectSignal(selectActiveWindow);
  readonly showTray = this.store.selectSignal(selectTrayHasItems);

  ngOnInit(): void {
    this.barService.init();
  }

  openSoundModal(): void {
    this.modalService.open(SoundModalComponent);
  }

  openCalendar(): void {
    this.modalService.open(CalendarModalComponent);
  }

  openPowerMenu(): void {
    this.modalService.open(PowerMenuModalComponent, {
      withBackdrop: true,
      position: ModalPosition.Center,
    });
  }
}
