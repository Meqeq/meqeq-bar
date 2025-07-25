import { Component, computed, inject } from '@angular/core';
import { SoundService } from '../../sound/sound.service';
import {
  LucideAngularModule,
  Volume,
  Volume1,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { DecimalPipe } from '@angular/common';

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [LucideAngularModule, DecimalPipe],
})
export class SoundComponent {
  readonly soundService = inject(SoundService);

  readonly sinkIcon = computed(() => {
    if (this.soundService.defaultSink()?.muted) return VolumeOff;

    if (this.soundService.defaultVolume() > 50) return Volume2;

    if (this.soundService.defaultVolume() > 10) return Volume1;

    return Volume;
  });
}
