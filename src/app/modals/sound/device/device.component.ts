import {
  ChangeDetectionStrategy,
  Component,
  computed,
  inject,
  input,
  signal,
} from '@angular/core';

import { Store } from '@ngrx/store';
import { PwDeviceExtended } from '../../../reducers/pipewire/pipewire.schema';
import { NodeHeaderComponent } from '../common/node-header/node-header.component';
import { VolumeSliderComponent } from '../common/volume-slider/volume-slider.component';
import { PipewireActions } from '../../../reducers/pipewire/pipewire.actions';
import { DeviceConfigComponent } from '../common/device-config/device-config.component';
import { selectDefaultSink } from '../../../reducers/pipewire/pipewire.selectors';

@Component({
  selector: 'app-device',
  templateUrl: './device.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [NodeHeaderComponent, VolumeSliderComponent, DeviceConfigComponent],
})
export class DeviceComponent {
  private readonly store = inject(Store);

  readonly device = input.required<PwDeviceExtended>();

  readonly type = input.required<'input' | 'output'>();

  private readonly defaultSink = this.store.selectSignal(selectDefaultSink);
  private readonly defaultSource = this.store.selectSignal(selectDefaultSink);

  readonly enumRoutes = computed(() => {
    return Object.values(this.device().enumRoutes[this.type()]);
  });

  readonly enumProfiles = computed(() => {
    return Object.values(this.device().enumProfiles);
  });

  readonly currentRoute = computed(() => {
    return this.device().route[this.type()];
  });

  readonly currentProfile = computed(() => {
    return this.device().profile;
  });

  readonly volume = computed(() => {
    return this.currentRoute()?.volume[0] ?? 0;
  });

  readonly default = computed(() => {
    if (this.type() === 'input') return this.device() === this.defaultSource();

    return this.device() === this.defaultSink();
  });

  readonly disabled = computed(() => !this.currentRoute());

  readonly showSettings = signal(false);

  changeMute(mute: boolean): void {
    const props = {
      id: this.device().id,
      routeType: this.type(),
    };

    this.store.dispatch(
      mute
        ? PipewireActions.muteDevice(props)
        : PipewireActions.unmuteDevice(props),
    );
  }

  changeVolume(volume: number): void {
    this.store.dispatch(
      PipewireActions.changeDeviceVolume({
        id: this.device().id,
        routeType: this.type(),
        volume,
      }),
    );
  }

  changeRoute(routeIndex: number): void {
    this.store.dispatch(
      PipewireActions.setDeviceRoute({
        id: this.device().id,
        routeType: this.type(),
        routeIndex,
      }),
    );
  }

  changeProfile(profileIndex: number): void {
    this.store.dispatch(
      PipewireActions.setDeviceProfile({
        id: this.device().id,
        profileIndex,
      }),
    );
  }

  setDefault(): void {
    const props = { id: this.device().id };
    this.store.dispatch(
      this.type() === 'output'
        ? PipewireActions.setDefaultSink(props)
        : PipewireActions.setDefaultSource(props),
    );
  }
}
