import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { PwDevice, PwDeviceProfile } from '../sound-new.schema';
import { SoundNewService } from '../sound-new.service';
import { JsonPipe } from '@angular/common';
import { ComboComponent } from '../../common/combo/combo.component';

import { FormsModule } from '@angular/forms';
import { LucideAngularModule } from 'lucide-angular';
import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-device-config',
  templateUrl: './device-config.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FormsModule, JsonPipe, LucideAngularModule, ComboComponent],
})
export class DeviceConfigComponent {
  private readonly soundService = inject(SoundNewService);

  readonly device = input.required<PwDevice>();

  readonly enumProfiles = computed(() => {
    const enumProfiles = this.soundService
      .deviceEnumProfiles()
      .get(this.device().id);

    if (!enumProfiles) return [];

    return enumProfiles.values().toArray();
  });

  readonly currentProfile = computed(() => {
    return this.soundService.deviceProfile().get(this.device().id);
  });

  readonly profileControl = signal(-1);

  constructor() {
    effect(() => {
      if (this.currentProfile())
        this.profileControl.set(this.currentProfile()!.index);
    });
  }

  changeProfile(profile: PwDeviceProfile): void {
    console.log(this.enumProfiles(), this.currentProfile(), profile);
    if (this.currentProfile()?.index === profile.index) return;

    invoke('set_device_profile', {
      id: this.device().id,
      profileIndex: profile.index,
    });
  }
}
