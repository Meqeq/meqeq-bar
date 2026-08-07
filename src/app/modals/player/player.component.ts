import {
  Component,
  computed,
  inject,
  linkedSignal,
  signal,
} from '@angular/core';
import { Store } from '@ngrx/store';
import {
  selectActivePlayer,
  selectPlayers,
  selectSelectedPlayer,
} from '../../reducers/player/player.selectors';
import { JsonPipe } from '@angular/common';
import {
  ArrowRight,
  LucideAngularModule,
  Music,
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
import { PlayerActions } from '../../reducers/player/player.actions';
import {
  LoopStatus,
  PlaybackStatus,
} from '../../reducers/player/player.schema';
import { PlayerInfoComponent } from './info/player-info.component';
import { PlayerProgressComponent } from './progress/player-progress.component';
import { PlayerControlComponent } from './control/player-control.component';

@Component({
  selector: 'app-player',
  templateUrl: './player.component.html',
  imports: [
    LucideAngularModule,
    PlayerInfoComponent,
    PlayerProgressComponent,
    PlayerControlComponent,
  ],
})
export class PlayerModalComponent {
  private readonly store = inject(Store);

  readonly players = this.store.selectSignal(selectPlayers);
  readonly activePlayer = this.store.selectSignal(selectActivePlayer);
  readonly selectedPlayer = this.store.selectSignal(selectSelectedPlayer);

  // readonly currentlyPlaying = computed(() => {
  //   return (
  //     this.selectedPlayer()?.metadata.playbackStatus === PlaybackStatus.Playing
  //   );
  // });

  // readonly per = linkedSignal(() => {
  //   return Math.max(
  //     ((this.selectedPlayer()?.metadata.position ?? 0) /
  //       (this.selectedPlayer()?.metadata.length ?? 1)) *
  //       100,

  //     1,
  //   );
  // });

  // readonly showProgressAnimation = signal(true);

  // selectPlayer(name: string): void {
  //   this.store.dispatch(PlayerActions.selectedPlayer({ name }));
  // }

  // play(): void {
  //   this.store.dispatch(
  //     PlayerActions.play({
  //       name: this.selectedPlayer()?.name ?? '',
  //     }),
  //   );
  // }

  // pause(): void {
  //   this.store.dispatch(
  //     PlayerActions.pause({
  //       name: this.selectedPlayer()?.name ?? '',
  //     }),
  //   );
  // }

  // next(): void {
  //   this.store.dispatch(
  //     PlayerActions.next({
  //       name: this.selectedPlayer()?.name ?? '',
  //     }),
  //   );
  // }

  // prev(): void {
  //   this.store.dispatch(
  //     PlayerActions.prev({
  //       name: this.selectedPlayer()?.name ?? '',
  //     }),
  //   );
  // }

  // seek(event: MouseEvent): void {
  //   console.log(
  //     event,
  //     event.offsetX,
  //     (event.currentTarget as HTMLDivElement).clientWidth,
  //   );

  //   const fraction =
  //     event.offsetX / (event.currentTarget as HTMLDivElement).clientWidth;

  //   this.store.dispatch(
  //     PlayerActions.seek({
  //       name: this.selectedPlayer()?.name ?? '',
  //       position:
  //         Math.round(fraction * (this.selectedPlayer()?.metadata.length ?? 0)) -
  //         (this.selectedPlayer()?.metadata.position ?? 0),
  //     }),
  //   );

  //   this.per.set(fraction * 100);

  //   this.showProgressAnimation.set(false);

  //   setTimeout(() => {
  //     this.showProgressAnimation.set(true);
  //   });
  // }

  // readonly loopIcon = computed(() => {
  //   switch (this.selectedPlayer()?.metadata.loopStatus) {
  //     case LoopStatus.None:
  //       return Repeat;
  //     case LoopStatus.Track:
  //       return Repeat1;
  //     case LoopStatus.Playlist:
  //       return Repeat2;

  //     default:
  //       return Repeat;
  //   }
  // });

  // readonly shuffleIcon = computed(() => {
  //   if (this.selectedPlayer()?.metadata.shuffle) return Shuffle;
  //   else return ArrowRight;
  // });

  readonly icons = {
    playing: Music,
    play: Play,
    pause: Pause,
    stop: Square,
    next: SkipForward,
    prev: SkipBack,
  };
}
