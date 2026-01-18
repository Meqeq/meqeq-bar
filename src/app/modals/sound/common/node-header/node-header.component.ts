import {
  ChangeDetectionStrategy,
  Component,
  computed,
  input,
  model,
} from '@angular/core';
import {
  Check,
  LucideAngularModule,
  Mic,
  MicOff,
  Settings,
  Volume2,
  VolumeOff,
} from 'lucide-angular';

export type NodeDisplayType = 'output' | 'input' | 'playback' | 'recording';

@Component({
  selector: 'app-node-header',
  templateUrl: './node-header.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [LucideAngularModule],
})
export class NodeHeaderComponent {
  readonly title = input.required<string>();
  readonly type = input.required<NodeDisplayType>();

  readonly description = input('');
  readonly disabled = input(false);

  readonly default = model(false);
  readonly muted = model(false);

  readonly showSettings = model(false);

  readonly muteIcon = computed(() => {
    switch (this.type()) {
      case 'output':
      case 'playback':
        return this.muted() ? VolumeOff : Volume2;
      case 'input':
      case 'recording':
        return this.muted() ? MicOff : Mic;
    }
  });

  readonly icons = {
    check: Check,
    settings: Settings,
  } as const;
}
