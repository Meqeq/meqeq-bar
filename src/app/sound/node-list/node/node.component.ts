import {
  Component,
  computed,
  effect,
  inject,
  input,
  signal,
} from '@angular/core';
import { FormsModule } from '@angular/forms';
import { PwNode, SoundService } from '../../sound.service';
import {
  LucideAngularModule,
  Mic,
  MicOff,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { invoke } from '@tauri-apps/api/core';

export type NodeType = 'source' | 'sink' | 'stream';

@Component({
  selector: 'app-node',
  templateUrl: './node.component.html',
  styleUrl: './node.component.scss',
  imports: [FormsModule, LucideAngularModule],
})
export class NodeComponent {
  private readonly soundService = inject(SoundService);

  readonly node = input.required<PwNode>();

  readonly type = input.required<NodeType>();

  readonly value = signal(0);

  readonly displayName = computed(() => {
    if (this.node().nick) return this.node().nick;

    return this.node().name;
  });

  readonly props = computed(() => {
    const propsMap = this.soundService.nodespwprops();

    if (propsMap) {
      const props = propsMap.get(this.node().id);

      if (props) return props;
    }

    return {
      id: this.node().id,
      volume: [0, 0],
      muted: false,
    };
  });

  readonly volume = computed(() => {
    return this.props().volume.map((v) => Math.round(v * 100));
  });

  readonly isDefault = computed(() => {
    if (this.type() === 'sink')
      return this.node().name === this.soundService.defaultSinkName().name;
    return this.node().name === this.soundService.defaultSourceName().name;
  });

  constructor() {
    effect(() => {
      this.value.set(this.volume()[0] / 100);
    });
  }

  setDefault() {
    console.log('AAAAA');
    if (this.type() === 'source')
      invoke('set_default_source', {
        source: JSON.stringify({ name: this.node().name }),
      });

    if (this.type() === 'sink') {
      invoke('set_default_sink', {
        sink: JSON.stringify({ name: this.node().name }),
      });
    }
  }

  changeVolume() {
    invoke('set_node_props', {
      id: this.node().id,
      channelVolumes: [this.value(), this.value()],
      mute: false,
    });
  }

  readonly icons = {
    ['sink' as NodeType]: {
      muted: VolumeOff,
      unmuted: Volume2,
    },
    ['source' as NodeType]: {
      muted: MicOff,
      unmuted: Mic,
    },
  };
}
