import { Component, computed, inject } from '@angular/core';

import {
  LucideAngularModule,
  Volume,
  Volume1,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { DecimalPipe } from '@angular/common';
import { SoundService } from '../../sound/sound.service';

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [LucideAngularModule, DecimalPipe],
})
export class SoundComponent {
  readonly soundService = inject(SoundService);

  readonly route = computed(() => {
    const defaultDevice = this.soundService.defaultSinkDevice()?.id;

    if (!defaultDevice) return undefined;

    return this.soundService.deviceRoute().get(defaultDevice);
  });

  readonly volume = computed(() => {
    return Math.round((this.route()?.output?.volume[0] ?? 0) * 100);
  });

  readonly sinkIcon = computed(() => {
    if (this.route()?.output?.mute || this.volume() < 1) return VolumeOff;

    if (this.volume() > 50) return Volume2;

    if (this.volume() > 10) return Volume1;

    return Volume;
  });
}
