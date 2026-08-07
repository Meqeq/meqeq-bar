import {
  Component,
  computed,
  effect,
  inject,
  linkedSignal,
} from '@angular/core';

import { Store } from '@ngrx/store';
import {
  selectAbilitiesForSelectedPlayer,
  selectMetadataForSelectedPlayer,
  selectSelectedPlayer,
} from '../../../reducers/player/player.selectors';
import { PlayerActions } from '../../../reducers/player/player.actions';
import { DatePipe } from '@angular/common';

@Component({
  selector: 'app-player-progress',
  templateUrl: './player-progress.component.html',
  providers: [DatePipe],
})
export class PlayerProgressComponent {
  private readonly store = inject(Store);
  private readonly datePipe = inject(DatePipe);

  readonly abilities = this.store.selectSignal(
    selectAbilitiesForSelectedPlayer,
  );
  readonly metadata = this.store.selectSignal(selectMetadataForSelectedPlayer);
  readonly selectedPlayer = this.store.selectSignal(selectSelectedPlayer);

  readonly canSeek = computed(() => {
    return this.abilities().seek;
  });

  readonly progress = linkedSignal(() => {
    return (this.metadata().position / this.metadata().length) * 100;
  });

  readonly current = computed(() => {
    return this.datePipe.transform(
      Math.round(this.metadata().position / 1000),
      'mm:ss',
    );
  });

  readonly total = computed(() => {
    return this.datePipe.transform(
      Math.round(this.metadata().length / 1000),
      'mm:ss',
    );
  });

  readonly title = computed(() => this.metadata().title);

  readonly progressAnimationClasses = linkedSignal(() => {
    this.progress();
    return 'transition-width duration-1000 ease-linear';
  });

  constructor() {
    effect(() => {
      this.title();

      this.progressAnimationClasses.set('');
    });
  }

  handleSeek(event: MouseEvent): void {
    if (!this.canSeek()) return;

    const fraction =
      event.offsetX / (event.currentTarget as HTMLDivElement).clientWidth;

    this.progress.set(fraction * 100);
    this.progressAnimationClasses.set('');

    this.store.dispatch(
      PlayerActions.seek({
        name: this.selectedPlayer(),
        position:
          Math.round(fraction * this.metadata().length) -
          this.metadata().position,
      }),
    );
  }
}
