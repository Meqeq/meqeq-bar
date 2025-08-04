import { Component, computed, inject, input } from '@angular/core';
import { PwDevice, SoundService } from '../../sound.service';
import { JsonPipe } from '@angular/common';
import { simpleDataSource } from '../../../common/simple-data-source';
import { of } from 'rxjs';
import { ComboComponent } from '../../../common/combo/combo.component';

@Component({
  selector: 'app-device',
  templateUrl: 'device.component.html',
  imports: [ComboComponent, JsonPipe],
})
export class DeviceComponent {
  private readonly soundService = inject(SoundService);

  readonly type = input.required<'input' | 'output' | 'config'>();
  readonly device = input.required<PwDevice>();

  readonly enumRoutes = computed(() => {
    return [
      ...(this.soundService.deviceRoutes().get(this.device().id)?.values() ??
        []),
    ];
  });

  readonly profileSource = simpleDataSource({
    request: () => {
      return {
        device: this.device().id,
      };
    },
    sourceFn: (request) => {
      return of([
        ...(this.soundService
          .deviceEnumProfiles()
          .get(this.device().id)
          ?.values() ?? []),
      ]);
    },
  });

  // readonly enumProfiles = computed(() => {
  //   return [
  //     ...this.soundService
  //       .deviceEnumProfiles()
  //       .values()
  //       .filter((profile) => profile.deviceId === this.device().id),
  //   ];
  // });
}
