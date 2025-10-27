import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { PwDevice, PwDeviceRoute } from '../sound-new.schema';
import { SoundNewService } from '../sound-new.service';
import { JsonPipe } from '@angular/common';
import { ComboComponent } from '../../common/combo/combo.component';

import { FormsModule } from '@angular/forms';
import {
  LucideAngularModule,
  Mic,
  MicOff,
  Settings,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { invoke } from '@tauri-apps/api/core';
import { PwNode } from '../../sound/sound.service';

@Component({
  selector: 'app-device',
  templateUrl: './device.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FormsModule, JsonPipe, LucideAngularModule, ComboComponent],
})
export class DeviceComponent {
  private readonly soundService = inject(SoundNewService);

  readonly device = input.required<PwDevice>();

  readonly type = input.required<'input' | 'output'>();

  readonly default = input(false);

  readonly enumRoutes = computed(() => {
    const enumRoutes = this.soundService
      .deviceEnumRoutes()
      .get(this.device().id);

    if (!enumRoutes) return [];

    return enumRoutes[this.type()].values().toArray();
  });

  readonly currentRoute = computed(() => {
    const route = this.soundService.deviceRoute().get(this.device().id);
    if (route) return route[this.type()];
    return undefined;
  });

  readonly volume = computed(() => {
    return Math.round((this.currentRoute()?.volume[0] ?? 0) * 100);
  });

  readonly value = signal(0);

  readonly isChanging = signal(false);

  readonly routeControl = signal<number | undefined>(undefined);

  readonly showSettings = signal(false);

  constructor() {
    effect(() => {
      // this.soundService.deviceEnumProfiles();
    });

    effect(() => {
      this.routeControl.set(this.currentRoute()?.index);
    });

    effect(() => {
      if (!this.isChanging())
        this.value.set(this.currentRoute()?.volume[0] ?? 0);
    });
  }

  changeMute(): void {
    invoke('set_device_mute', {
      id: this.device().id,
      routeIndex: this.currentRoute()?.index,
      routeDevice: this.currentRoute()?.devices[0],
      mute: !this.currentRoute()?.mute,
    });
  }

  changeVolume(): void {
    invoke('set_device_volume', {
      id: this.device().id,
      routeIndex: this.currentRoute()?.index,
      routeDevice: this.currentRoute()?.devices[0],
      channelVolumes: [this.value(), this.value()],
    });
  }

  changeRoute(route: PwDeviceRoute): void {
    if (this.currentRoute()?.index === route.index) return;

    invoke('set_device_route', {
      id: this.device().id,
      routeIndex: route.index,
      routeDevice: route.devices[0],
      channelVolumes: [this.value(), this.value()],
    });
  }

  setDefault(): void {
    let node: PwNode | undefined;

    this.soundService.nodes().forEach((n) => {
      if (n.deviceId === this.device().id) node = n;
    });

    if (!node) return;
    invoke(
      this.type() === 'output' ? 'set_default_sink' : 'set_default_source',
      {
        sink: JSON.stringify({ name: node.name }),
      },
    );
  }

  readonly icons = {
    input: {
      muted: MicOff,
      unmuted: Mic,
    },
    output: {
      muted: VolumeOff,
      unmuted: Volume2,
    },

    settings: Settings,
  } as const;
}
