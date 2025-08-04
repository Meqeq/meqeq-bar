import { Component, computed, inject, input } from '@angular/core';
import { SoundService } from '../sound.service';
import { JsonPipe } from '@angular/common';
import { DeviceComponent } from './device/device.component';

@Component({
  selector: 'app-devices-list',
  templateUrl: './devices-list.component.html',
  imports: [DeviceComponent, JsonPipe],
})
export class DevicesListComponent {
  private readonly soundService = inject(SoundService);

  readonly type = input.required<'input' | 'output' | 'config'>();

  readonly items = computed(() => {
    const devices = this.soundService.devices();
    if (!devices) return [];

    return [...devices.values()];
  });
}
