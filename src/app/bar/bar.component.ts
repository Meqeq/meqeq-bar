import { Component, inject } from '@angular/core';
import { BarService } from './bar.service';
import { WorkspacesComponent } from './workspaces/workspaces.component';
import { StartComponent } from './start/start.component';
import { ClockComponent } from './clock/clock.component';
import { PowerMenuComponent } from './power-menu/power-menu.component';
import { RouterOutlet } from '@angular/router';
import { SoundComponent } from './sound/sound.component';
import { fromTauriEvent } from '../common/tauri-utils';
import { map } from 'rxjs';
import { toSignal } from '@angular/core/rxjs-interop';
import { EthernetComponent } from './ethernet/ethernet.component';

interface ActiveWindow {
  class: string;
  title: string;
}

@Component({
  selector: 'app-bar',
  templateUrl: './bar.component.html',
  imports: [
    RouterOutlet,
    WorkspacesComponent,
    StartComponent,
    SoundComponent,
    ClockComponent,
    EthernetComponent,
    PowerMenuComponent,
  ],
  providers: [BarService],
})
export class BarComponent {
  readonly barService = inject(BarService);

  readonly activeWindow = toSignal(
    fromTauriEvent<ActiveWindow>('active_window_change').pipe(
      map((res) => res.title),
    ),
    {
      initialValue: '',
    },
  );

  ngOnInit(): void {
    this.barService.init();
  }
}
