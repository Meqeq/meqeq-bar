import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  input,
  signal,
} from '@angular/core';
import { PwDeviceProfile } from '../sound.schema';
import { ComboComponent } from '../../common/combo/combo.component';

import { FormsModule } from '@angular/forms';
import { LucideAngularModule } from 'lucide-angular';
import { invoke } from '@tauri-apps/api/core';
import { PwDeviceExtended } from '../../reducers/pipewire/pipewire.schema';

@Component({
  selector: 'app-device-config',
  templateUrl: './device-config.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FormsModule, LucideAngularModule, ComboComponent],
})
export class DeviceConfigComponent {
  readonly device = input.required<PwDeviceExtended>();

  readonly enumProfiles = computed(() => {
    return Object.values(this.device().enumProfiles);
  });

  readonly currentProfile = computed(() => {
    return this.device().profile;
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
