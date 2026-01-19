import { createFeature, createReducer, on } from '@ngrx/store';
import { BarActions } from './bar.actions';

export const barFeatureKey = 'bar';

export type Layer = 'top' | 'bottom';

export interface State {
  layer: Layer;
}

export const initialState: State = {
  layer: 'bottom',
};

export const reducer = createReducer(
  initialState,
  on(BarActions.layerSet, (state, { layer }) => ({
    ...state,
    layer,
  })),
);

export const barFeature = createFeature({
  name: barFeatureKey,
  reducer,
});
