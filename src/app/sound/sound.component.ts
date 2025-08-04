import { Component, computed, inject, signal } from '@angular/core';
import { SoundService } from './sound.service';
import { NodeListComponent } from './node-list/node-list.component';
import { JsonPipe } from '@angular/common';
import { DevicesListComponent } from './devices-list/devices-list.component';

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [NodeListComponent, DevicesListComponent, JsonPipe],
})
export class SoundComponent {
  readonly soundService = inject(SoundService);

  readonly tab = signal<'devices' | 'streams' | 'config'>('config');

  readonly items = computed(() => {
    return [...(this.soundService.nodespw()?.values() ?? [])];
  });

  readonly devices = computed(() => {
    return [...(this.soundService.deviceEnumProfiles()?.values() ?? [])];
  });
}
