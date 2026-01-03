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

@Component({
  selector: 'app-bar',
  templateUrl: './bar.component.html',
  imports: [
    RouterOutlet,
    WorkspacesComponent,
    StartComponent,
    SoundComponent,
    ClockComponent,
    TrayComponent,
    EthernetComponent,
    PowerMenuComponent,
  ],
  providers: [BarService],
})
export class BarComponent {
  readonly barService = inject(BarService);
  private readonly store = inject(Store);

  readonly activeWindow = this.store.selectSignal(selectActiveWindow);
  readonly showTray = this.store.selectSignal(selectTrayHasItems);

  ngOnInit(): void {
    this.barService.init();
  }
}
