import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { PwDevice } from '../sound-new.schema';
import { SoundNewService } from '../sound-new.service';
import { JsonPipe } from '@angular/common';
import { ComboComponent } from '../../common/combo/combo.component';

import { FormsModule } from '@angular/forms';
import {
  LucideAngularModule,
  Settings,
  Volume2,
  VolumeOff,
} from 'lucide-angular';

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
      for (let kek of this.enumRoutes()) {
        console.log(kek);
      }
    });

    effect(() => {
      this.routeControl.set(this.currentRoute()?.index);
    });

    effect(() => {
      if (!this.isChanging())
        this.value.set(this.currentRoute()?.volume[0] ?? 0);
    });
  }

  changeVolume(): void {
    console.log('CHANGE');
  }

  readonly icons = {
    muted: VolumeOff,
    unmuted: Volume2,
    settings: Settings,
  };
}
