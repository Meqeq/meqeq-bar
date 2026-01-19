import * as fromBar from './bar.reducer';
import { selectBarState } from './bar.selectors';

describe('Bar Selectors', () => {
  it('should select the feature state', () => {
    const result = selectBarState({
      [fromBar.barFeatureKey]: {}
    });

    expect(result).toEqual({});
  });
});
