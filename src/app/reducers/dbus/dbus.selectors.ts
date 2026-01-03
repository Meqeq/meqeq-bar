import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromDbus from './dbus.reducer';

export const selectDbusState = createFeatureSelector<fromDbus.State>(
  fromDbus.dbusFeatureKey,
);

export const selectTrayItems = createSelector(
  selectDbusState,
  (state) => state.trayItems,
);

export const selectTrayItemsArray = createSelector(selectTrayItems, (items) =>
  Object.values(items),
);

export const selectTrayHasItems = createSelector(
  selectTrayItemsArray,
  (items) => !!items.length,
);
