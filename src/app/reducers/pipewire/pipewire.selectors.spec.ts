import * as fromPipewire from './pipewire.reducer';
import { selectPipewireState } from './pipewire.selectors';

describe('Pipewire Selectors', () => {
  it('should select the feature state', () => {
    const result = selectPipewireState({
      [fromPipewire.pipewireFeatureKey]: {}
    });

    expect(result).toEqual({});
  });
});
