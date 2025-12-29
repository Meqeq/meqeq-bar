import * as fromHyprland from './hyprland.reducer';
import { selectHyprlandState } from './hyprland.selectors';

describe('Hyprland Selectors', () => {
  it('should select the feature state', () => {
    const result = selectHyprlandState({
      [fromHyprland.hyprlandFeatureKey]: {}
    });

    expect(result).toEqual({});
  });
});
