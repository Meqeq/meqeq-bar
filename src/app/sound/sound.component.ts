import { Component, computed, inject, signal } from '@angular/core';
import { SoundService } from './sound.service';
import { NodeListComponent } from './node-list/node-list.component';

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [NodeListComponent],
})
export class SoundComponent {
  readonly soundService = inject(SoundService);

  readonly tab = signal<'devices' | 'streams'>('devices');

  readonly items = computed(() => {
    return [...(this.soundService.nodespw()?.values() ?? [])];
  });
}
