import * as fromDbus from './dbus.reducer';
import { selectDbusState } from './dbus.selectors';

describe('Dbus Selectors', () => {
  it('should select the feature state', () => {
    const result = selectDbusState({
      [fromDbus.dbusFeatureKey]: {}
    });

    expect(result).toEqual({});
  });
});
