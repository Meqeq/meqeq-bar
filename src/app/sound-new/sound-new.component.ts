import { Component, computed, effect, inject, signal } from '@angular/core';
import { SoundNewService } from './sound-new.service';
import { DeviceComponent } from './device/device.component';
import { NodeComponent } from './node/node.component';
import { DeviceConfigComponent } from './device-config/device-config.component';
import { JsonPipe } from '@angular/common';

const menuOptions = [
  'output',
  'input',
  'playback',
  'recording',
  'config',
] as const;

type MenuOption = (typeof menuOptions)[number];

@Component({
  selector: 'app-sound-new',
  templateUrl: './sound-new.component.html',
  imports: [DeviceComponent, NodeComponent, DeviceConfigComponent, JsonPipe],
})
export class SoundNewComponent {
  readonly soundService = inject(SoundNewService);

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
