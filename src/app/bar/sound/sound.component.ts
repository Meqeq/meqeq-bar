import { Component, computed, inject } from '@angular/core';

import {
  LucideAngularModule,
  Volume,
  Volume1,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { DecimalPipe } from '@angular/common';
import { Store } from '@ngrx/store';
import { selectDefaultSink } from '../../reducers/pipewire/pipewire.selectors';

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [LucideAngularModule, DecimalPipe],
})
export class SoundComponent {
  private readonly store = inject(Store);

  readonly defaultDevice = this.store.selectSignal(selectDefaultSink);

  readonly volume = computed(() => {
    return Math.round(
      (this.defaultDevice()?.route.output?.volume[0] ?? 0) * 100,
    );
  });

  readonly sinkIcon = computed(() => {
    if (this.defaultDevice()?.route?.output?.mute || this.volume() < 1)
      return VolumeOff;

    if (this.volume() > 50) return Volume2;

    if (this.volume() > 10) return Volume1;

    return Volume;
  });
}
