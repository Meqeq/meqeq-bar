export interface MediaPlayerResponse {
  name: string;
  identity: string;
}

export enum PlaybackStatus {
  Playing = 'playing',
  Paused = 'paused',
  Stopped = 'stopped',
}

export enum LoopStatus {
  None = 'none',
  Track = 'track',
  Playlist = 'playlist',
}

export enum PlayerAbility {
  GoNext = 'goNext',
  GoPrevious = 'goPrevious',
  Play = 'play',
  Pause = 'pause',
  Seek = 'seek',
  Control = 'control',
}

export type MetadataMapEntry =
  | {
      signature: 'i' | 'd' | 't';
      value: number;
    }
  | {
      signature: 's';
      value: string;
    }
  | {
      signature: 'as';
      value: string[];
    };

export type MetadataMap = Record<string, MetadataMapEntry>;

export interface PlayerInfo {
  title: string;
  artist: string;
  cover: string;
  album: string;
}

export interface Metadata extends PlayerInfo {
  playbackStatus: PlaybackStatus;
  loopStatus: LoopStatus;

  length: number;
  position: number;
  shuffle: boolean;
}

export interface MediaPlayer extends MediaPlayerResponse {
  metadata: Metadata;
  abilities: Record<PlayerAbility, boolean>;
}
