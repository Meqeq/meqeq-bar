import { createFeature, createReducer, on } from '@ngrx/store';
import { PlayerActions } from './player.actions';
import {
  LoopStatus,
  MediaPlayerResponse,
  Metadata,
  PlaybackStatus,
  PlayerAbility,
} from './player.schema';
import { parseMetadata } from './player.utils';

export const playerFeatureKey = 'player';

export interface State {
  activePlayer: string;
  selectedPlayer: string;

  players: Record<string, MediaPlayerResponse>;
  metadatas: Record<string, Metadata>;
  abilities: Record<string, Record<PlayerAbility, boolean>>;
}

export const initialState: State = {
  players: {},
  activePlayer: '',
  selectedPlayer: '',
  metadatas: {},
  abilities: {},
};

export const reducer = createReducer(
  initialState,
  on(PlayerActions.new, (state, { player }) => ({
    ...state,
    activePlayer: player.name,
    selectedPlayer: player.name,
    players: { ...state.players, [player.name]: player },
    metadatas: {
      ...state.metadatas,
      [player.name]: {
        playbackStatus: PlaybackStatus.Stopped,
        loopStatus: LoopStatus.None,
        title: '',
        artist: '',
        cover: '',
        album: '',
        length: 0,
        shuffle: false,
        position: 0,
      },
    },
    abilities: {
      [player.name]: {
        [PlayerAbility.GoNext]: false,
        [PlayerAbility.GoPrevious]: false,
        [PlayerAbility.Play]: false,
        [PlayerAbility.Pause]: false,
        [PlayerAbility.Seek]: false,
        [PlayerAbility.Control]: false,
      },
    },
  })),
  on(PlayerActions.disconnect, (state, { name }) => {
    const { [name]: _removed, ...players } = state.players;
    const { [name]: _removed3, ...metadatas } = state.metadatas;

    console.log('DISCONNECT', name, players, metadatas);

    return {
      ...state,
      activePlayer: '',
      selectedPlayer: '',
      players,
      metadatas,
    };
  }),
  on(PlayerActions.playbackStatus, (state, { name, status }) => ({
    ...state,
    activePlayer: name,
    selectedPlayer: name,
    metadatas: {
      ...state.metadatas,
      [name]: {
        ...state.metadatas[name],
        playbackStatus: status,
      },
    },
  })),
  on(PlayerActions.activePlayer, (state, { name }) => ({
    ...state,
    activePlayer: name,
  })),
  on(PlayerActions.selectedPlayer, (state, { name }) => ({
    ...state,
    selectedPlayer: name,
  })),
  on(PlayerActions.metadata, (state, { name, metadata }) => ({
    ...state,
    activePlayer: name,
    metadatas: {
      ...state.metadatas,
      [name]: {
        ...state.metadatas[name],
        ...parseMetadata(metadata),
      },
    },
  })),
  on(PlayerActions.loopStatusUpdated, (state, { name, status }) => ({
    ...state,
    metadatas: {
      ...state.metadatas,
      [name]: {
        ...state.metadatas[name],
        loopStatus: status,
      },
    },
  })),
  on(PlayerActions.shuffleUpdated, (state, { name, property }) => ({
    ...state,
    metadatas: {
      ...state.metadatas,
      [name]: {
        ...state.metadatas[name],
        shuffle: property,
      },
    },
  })),
  on(PlayerActions.abilityUpdated, (state, { name, ability, value }) => ({
    ...state,
    abilities: {
      ...state.abilities,
      [name]: {
        ...state.abilities[name],
        [ability]: value,
      },
    },
  })),
  on(PlayerActions.positionUpdated, (state, { name, position }) => ({
    ...state,
    metadatas: {
      ...state.metadatas,
      [name]: {
        ...state.metadatas[name],
        position,
      },
    },
  })),
);

export const playerFeature = createFeature({
  name: playerFeatureKey,
  reducer,
});
