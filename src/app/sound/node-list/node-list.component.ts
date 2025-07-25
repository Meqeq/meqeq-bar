import { Component, computed, effect, inject, input } from '@angular/core';
import { SoundService } from '../sound.service';
import { NodeComponent, NodeType } from './node/node.component';

@Component({
  selector: 'app-node-list',
  templateUrl: './node-list.component.html',
  imports: [NodeComponent],
})
export class NodeListComponent {
  private readonly soundService = inject(SoundService);

  readonly type = input.required<NodeType>({});

  readonly mediaClass = computed(() => {
    switch (this.type()) {
      case 'source':
        return 'Audio/Source';
      case 'sink':
        return 'Audio/Sink';
      case 'stream':
        return 'Stream/Output/Audio';
    }
  });

  readonly items = computed(() => {
    const nodes = this.soundService.nodespw();
    if (!nodes) return [];

    const filtered = nodes
      .values()
      .filter((node) => node.class === this.mediaClass());

    return [...filtered];
  });

  e = effect(() => {
    console.log(this.items());
  });
}
