import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { PwNode } from '../sound.schema';
import { SoundService } from '../sound.service';
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
  styles: [
    `
      .slider {
        -webkit-appearance: none;
        appearance: none;
        width: 100%;
        height: 0.5rem;
        border-radius: 9999px;
        background: var(--color-neutral);
        outline: none;
        cursor: pointer;
        --val: 50%;
      }

      .slider::-webkit-slider-runnable-track {
        height: 0.5rem;
        border-radius: 9999px;
        background: linear-gradient(
          to right,
          var(--color-primary) var(--val),
          var(--color-neutral) var(--val)
        );
      }

      .slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 1rem;
        height: 1rem;
        border-radius: 9999px;
        background: var(--color-primary);
        border: 2px solid var(--color-primary-content);
        box-shadow: 0 0 2px rgba(0, 0, 0, 0.3);
        margin-top: -0.25rem;
        transition:
          background 0.2s,
          transform 0.1s;
      }

      .slider:active::-webkit-slider-thumb {
        outline: 2px solid var(--color-primary);
        outline-offset: 2px;
      }
    `,
  ],
})
export class NodeComponent {
  private readonly soundService = inject(SoundService);

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
