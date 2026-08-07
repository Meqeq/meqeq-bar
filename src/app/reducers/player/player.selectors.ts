import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromPlayer from './player.reducer';
import { selectRouteParam } from '../router/router.selectors';
import { MediaPlayer, PlayerInfo } from './player.schema';

export const selectPlayerState = createFeatureSelector<fromPlayer.State>(
  fromPlayer.playerFeatureKey,
);

export const selectPlayers = createSelector(selectPlayerState, (state) => {
  return Object.values(state.players);
});

export const selectCurrentlyPlaying = createSelector(
  selectPlayers,
  (players) => !!players.length,
);

export const selectActivePlayer = createSelector(selectPlayerState, (state) => {
  return state.activePlayer;
});

// export const selectSelectedPlayer = createSelector(
//   selectPlayerState,
//   (state) => {
//     if (state.selectedPlayer === '') return undefined;
//     console.log(state);
//     const { name, identity } = state.players[state.selectedPlayer];

//     const metadata = state.metadatas[name];
//     const abilities = state.abilities[name];

//     if (!metadata) throw new Error(`There is no metadata related to ${name}`);

//     if (!abilities) throw new Error(`There is no abilities related to ${name}`);

//     const result: MediaPlayer = {
//       name,
//       identity,
//       metadata,
//       abilities,
//     };

//     return result;
//   },
// );

export const selectSelectedPlayer = createSelector(
  selectPlayerState,
  (state) => state.selectedPlayer,
);

export const selectMetadatas = createSelector(
  selectPlayerState,
  (state) => state.metadatas,
);

export const selectAbilities = createSelector(
  selectPlayerState,
  (state) => state.abilities,
);

export const selectAbilitiesForSelectedPlayer = createSelector(
  selectSelectedPlayer,
  selectAbilities,
  (player, abilities) => {
    const result = abilities[player];

    if (!abilities)
      throw new Error(`There is no abilities related to ${player}`);

    return result;
  },
);

export const selectMetadataForSelectedPlayer = createSelector(
  selectSelectedPlayer,
  selectMetadatas,
  (player, metadatas) => {
    const result = metadatas[player];

    if (!result) throw new Error(`There is no metadata related to ${player}`);

    return result;
  },
);

export const selectInfoForSelectedPlayer = createSelector(
  selectMetadataForSelectedPlayer,
  (metadata) => {
    const result: PlayerInfo = {
      title: metadata.title,
      cover: metadata.cover,
      album: metadata.album,
      artist: metadata.artist,
    };

    return result;
  },
);
