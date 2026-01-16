import { createActionGroup, emptyProps, props } from '@ngrx/store';

export const BarActions = createActionGroup({
  source: 'Bar',
  events: {
    'Layer set': props<{ layer: 'top' | 'bottom' }>(),
    'Set top layer': emptyProps(),
    'Set bottom layer': emptyProps(),
  },
});
