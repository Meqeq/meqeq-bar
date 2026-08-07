import { Component, computed, inject } from '@angular/core';
import { Store } from '@ngrx/store';
import {
  selectActivePlayer,
  selectCurrentlyPlaying,
} from '../../reducers/player/player.selectors';
import { PlaybackStatus } from '../../reducers/player/player.schema';
import {
  CirclePause,
  CirclePlay,
  CircleStop,
  LoaderCircle,
  LucideAngularModule,
  Music,
} from 'lucide-angular';

@Component({
  selector: 'app-player',
  templateUrl: './player.component.html',
  imports: [LucideAngularModule],
})
export class PlayerComponent {
  readonly icon = Music;
}
