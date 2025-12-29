import { Component, inject, signal } from '@angular/core';
import { DeviceComponent } from './device/device.component';
import { NodeComponent } from './node/node.component';
import { DeviceConfigComponent } from './device-config/device-config.component';
import { Store } from '@ngrx/store';
import {
  selectDefaultSink,
  selectDefaultSource,
  selectDevicesList,
  selectPlaybacks,
  selectRecordings,
} from '../reducers/pipewire/pipewire.selectors';

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
  imports: [DeviceComponent, NodeComponent, DeviceConfigComponent],
})
export class SoundComponent {
  private readonly store = inject(Store);

  readonly tab = signal<MenuOption>('output');

  readonly devices = this.store.selectSignal(selectDevicesList);

  readonly defaultSink = this.store.selectSignal(selectDefaultSink);
  readonly defaultSource = this.store.selectSignal(selectDefaultSource);

  readonly playbacks = this.store.selectSignal(selectPlaybacks);
  readonly recordings = this.store.selectSignal(selectRecordings);

  readonly menuOptions = [
    { value: 'output', label: 'Wyjścia' },
    { value: 'input', label: 'Wejścia' },
    { value: 'playback', label: 'Odtwarzanie' },
    { value: 'recording', label: 'Nagrywanie' },
    { value: 'config', label: 'Konfiguracja' },
  ] as const;
}
