import { Component, effect, inject, signal } from '@angular/core';
import { SoundNewService } from './sound-new.service';
import { DeviceComponent } from './device/device.component';

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
  imports: [DeviceComponent],
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

  constructor() {
    effect(() => {
      // console.log(this.soundService.deviceEnumRoutes());
    });
  }
}
