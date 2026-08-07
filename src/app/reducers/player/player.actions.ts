import { createActionGroup, props } from '@ngrx/store';
import {
  LoopStatus,
  MediaPlayerResponse,
  MetadataMap,
  PlaybackStatus,
  PlayerAbility,
} from './player.schema';

export const PlayerActions = createActionGroup({
  source: 'Player',
  events: {
    New: props<{ player: MediaPlayerResponse }>(),
    Disconnect: props<{ name: string }>(),
    'Playback Status': props<{
      name: string;
      status: PlaybackStatus;
    }>(),
    'Active Player': props<{ name: string }>(),
    'Selected Player': props<{ name: string }>(),
    Metadata: props<{ name: string; metadata: MetadataMap }>(),
    Play: props<{ name: string }>(),
    Pause: props<{ name: string }>(),
    Next: props<{ name: string }>(),
    Prev: props<{ name: string }>(),
    Seek: props<{ name: string; position: number }>(),
    Shuffle: props<{ name: string; shuffle: boolean }>(),
    LoopStatusUpdated: props<{ name: string; status: LoopStatus }>(),
    ShuffleUpdated: props<{ name: string; property: boolean }>(),
    PositionUpdated: props<{ name: string; position: number }>(),
    AbilityUpdated: props<{
      name: string;
      ability: PlayerAbility;
      value: boolean;
    }>(),
  },
});
