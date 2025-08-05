import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { PwNode } from '../sound-new.schema';
import { SoundNewService } from '../sound-new.service';
import { DecimalPipe, JsonPipe } from '@angular/common';
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

@Component({
  selector: 'app-node',
  templateUrl: './node.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    FormsModule,
    DecimalPipe,
    JsonPipe,
    LucideAngularModule,
    ComboComponent,
  ],
})
export class NodeComponent {
  private readonly soundService = inject(SoundNewService);

  readonly node = input.required<PwNode>();

  readonly type = input.required<'input' | 'output'>();

  readonly props = computed(() => {
    return this.soundService.nodesProps().get(this.node().id);
  });

  readonly volume = computed(() => {
    return this.props()?.volume[0] ?? 0;
  });

  readonly value = signal(0);
  readonly isChanging = signal(false);
  readonly routeControl = signal<number | undefined>(undefined);

  constructor() {
    effect(() => {
      if (!this.isChanging()) this.value.set(this.volume());
    });
  }

  changeVolume(): void {
    invoke('set_node_volume', {
      id: this.node().id,
      channelVolumes: [this.value(), this.value()],
    });
  }

  changeMute() {
    invoke('set_node_mute', {
      id: this.node().id,
      mute: !this.props()?.muted,
    });
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
