import { Component, inject } from '@angular/core';
import { BarService } from './bar.service';
import { WorkspacesComponent } from './workspaces/workspaces.component';
import { StartComponent } from './start/start.component';
import { ClockComponent } from './clock/clock.component';
import { PowerMenuComponent } from './power-menu/power-menu.component';
import { RouterOutlet } from '@angular/router';
import { SoundComponent } from './sound/sound.component';

@Component({
  selector: 'app-bar',
  templateUrl: './bar.component.html',
  imports: [
    RouterOutlet,
    WorkspacesComponent,
    StartComponent,
    SoundComponent,
    ClockComponent,
    PowerMenuComponent,
  ],
  providers: [BarService],
})
export class BarComponent {
  readonly barService = inject(BarService);

  ngOnInit(): void {
    this.barService.init();
  }
}
