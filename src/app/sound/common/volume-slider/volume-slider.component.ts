import {
  ChangeDetectionStrategy,
  Component,
  computed,
  input,
  model,
} from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-volume-slider',
  templateUrl: './volume-slider.component.html',
  styleUrl: './volume-slider.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FormsModule],
})
export class VolumeSliderComponent {
  readonly disabled = input(false);

  readonly volume = model(0);

  readonly volumeDisplay = computed(
    () => `${Math.round(this.volume() * 100)}%`,
  );
}
