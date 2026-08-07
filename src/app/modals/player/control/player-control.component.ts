import { Component, computed, inject } from '@angular/core';
import { Store } from '@ngrx/store';
import {
  ArrowRight,
  LucideAngularModule,
  Pause,
  Play,
  Repeat,
  Repeat1,
  Repeat2,
  Shuffle,
  SkipBack,
  SkipForward,
  Square,
} from 'lucide-angular';
import {
  selectAbilitiesForSelectedPlayer,
  selectMetadataForSelectedPlayer,
  selectSelectedPlayer,
} from '../../../reducers/player/player.selectors';
import {
  LoopStatus,
  PlaybackStatus,
} from '../../../reducers/player/player.schema';
import { PlayerActions } from '../../../reducers/player/player.actions';

@Component({
  selector: 'app-player-control',
  templateUrl: './player-control.component.html',
  imports: [LucideAngularModule],
})
export class PlayerControlComponent {
  private readonly store = inject(Store);

  readonly selectedPlayer = this.store.selectSignal(selectSelectedPlayer);
  readonly metadata = this.store.selectSignal(selectMetadataForSelectedPlayer);
  readonly abilities = this.store.selectSignal(
    selectAbilitiesForSelectedPlayer,
  );

  readonly currentlyPlaying = computed(() => {
    return this.metadata().playbackStatus === PlaybackStatus.Playing;
  });

  play(): void {
    this.store.dispatch(
      PlayerActions.play({
        name: this.selectedPlayer(),
      }),
    );
  }

  pause(): void {
    this.store.dispatch(
      PlayerActions.pause({
        name: this.selectedPlayer(),
      }),
    );
  }

  next(): void {
    this.store.dispatch(
      PlayerActions.next({
        name: this.selectedPlayer(),
      }),
    );
  }

  prev(): void {
    this.store.dispatch(
      PlayerActions.prev({
        name: this.selectedPlayer(),
      }),
    );
  }

  shuffle(): void {
    this.store.dispatch(
      PlayerActions.shuffle({
        name: this.selectedPlayer(),
        shuffle: !this.metadata().shuffle,
      }),
    );
  }

  readonly loopIcon = computed(() => {
    switch (this.metadata().loopStatus) {
      case LoopStatus.None:
        return Repeat;
      case LoopStatus.Track:
        return Repeat1;
      case LoopStatus.Playlist:
        return Repeat2;

      default:
        return Repeat;
    }
  });

  readonly shuffleIcon = computed(() => {
    if (this.metadata().shuffle) return Shuffle;
    else return ArrowRight;
  });

  readonly icons = {
    play: Play,
    pause: Pause,
    stop: Square,
    next: SkipForward,
    prev: SkipBack,
    shuffle: Shuffle,
  };
}
