import { Component, computed, effect, inject, signal } from '@angular/core';
import { DeviceComponent } from './device/device.component';
import { NodeComponent } from './node/node.component';
import { DeviceConfigComponent } from './device-config/device-config.component';
import { JsonPipe } from '@angular/common';
import { SoundService } from './sound.service';

const menuOptions = [
  'output',
  'input',
  'playback',
  'recording',
  'config',
] as const;

type MenuOption = (typeof menuOptions)[number];

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [DeviceComponent, NodeComponent, DeviceConfigComponent, JsonPipe],
})
export class SoundComponent {
  readonly soundService = inject(SoundService);

  readonly tab = signal<MenuOption>('output');

  readonly menuOptions = [
    { value: 'output', label: 'Wyjścia' },
    { value: 'input', label: 'Wejścia' },
    { value: 'playback', label: 'Odtwarzanie' },
    { value: 'recording', label: 'Nagrywanie' },
    { value: 'config', label: 'Konfiguracja' },
  ] as const;

  readonly playbacks = computed(() => {
    return this.soundService
      .nodes()
      .values()
      .filter((node) => node.class === 'Stream/Output/Audio')
      .toArray();
  });

  readonly recordings = computed(() => {
    return this.soundService
      .nodes()
      .values()
      .filter((node) => node.class === 'Stream/Input/Audio')
      .toArray();
  });

  constructor() {
    effect(() => {
      // console.log(this.soundService.deviceEnumRoutes());
    });
  }
}
