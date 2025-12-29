import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { PwDeviceRoute, PwNode } from '../sound.schema';
import { SoundService } from '../sound.service';
import { ComboComponent } from '../../common/combo/combo.component';

import { FormsModule } from '@angular/forms';
import {
  Check,
  LucideAngularModule,
  Mic,
  MicOff,
  Settings,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@ngrx/store';
import { PwDeviceExtended } from '../../reducers/pipewire/pipewire.schema';

@Component({
  selector: 'app-device',
  templateUrl: './device.component.html',
  styleUrl: './device.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FormsModule, LucideAngularModule, ComboComponent],
})
export class DeviceComponent {
  private readonly store = inject(Store);

  private readonly soundService = inject(SoundService);

  readonly device = input.required<PwDeviceExtended>();

  readonly type = input.required<'input' | 'output'>();

  readonly default = input(false);

  readonly enumRoutes = computed(() => {
    return Object.values(this.device().enumRoutes[this.type()]);
  });

  readonly currentRoute = computed(() => {
    return this.device().route[this.type()];
  });

  readonly volume = computed(() => {
    let v = this.isChanging()
      ? this.value()
      : (this.currentRoute()?.volume[0] ?? 0);

    return Math.round(v * 100);
  });

  readonly value = signal(0);

  readonly isChanging = signal(false);

  readonly routeControl = signal<number | undefined>(undefined);

  readonly showSettings = signal(false);

  constructor() {
    effect(() => {
      this.routeControl.set(this.currentRoute()?.index);
    });

    effect(() => {
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
    check: Check,
    settings: Settings,
  } as const;
}
