import { Component, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SoundNewService } from './sound-new/sound-new.service';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
})
export class AppComponent {
  private readonly _soundService = inject(SoundNewService);
}
