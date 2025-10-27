import { Component, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SoundService } from './sound/sound.service';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
})
export class AppComponent {
  private readonly _soundService = inject(SoundService);
}
