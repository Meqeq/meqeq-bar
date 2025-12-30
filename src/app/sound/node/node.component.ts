import {
  ChangeDetectionStrategy,
  Component,
  computed,
  inject,
  input,
} from '@angular/core';
import { NodeHeaderComponent } from '../common/node-header/node-header.component';
import { VolumeSliderComponent } from '../common/volume-slider/volume-slider.component';
import { PwNodeExtended } from '../../reducers/pipewire/pipewire.schema';
import { Store } from '@ngrx/store';
import { PipewireActions } from '../../reducers/pipewire/pipewire.actions';

@Component({
  selector: 'app-node',
  templateUrl: './node.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [NodeHeaderComponent, VolumeSliderComponent],
})
export class NodeComponent {
  private readonly store = inject(Store);

  readonly node = input.required<PwNodeExtended>();

  readonly type = input.required<'playback' | 'recording'>();

  readonly volume = computed(() => {
    return this.node().props?.volume[0] ?? 0;
  });

  readonly muted = computed(() => {
    return this.node().props?.muted ?? true;
  });

  changeVolume(volume: number): void {
    this.store.dispatch(
      PipewireActions.changeNodeVolume({ id: this.node().id, volume }),
    );
  }

  changeMute(muted: boolean) {
    this.store.dispatch(
      muted
        ? PipewireActions.muteNode({ id: this.node().id })
        : PipewireActions.unmuteNode({ id: this.node().id }),
    );
  }
}
