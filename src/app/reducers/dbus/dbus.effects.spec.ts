import { TestBed } from '@angular/core/testing';
import { provideMockActions } from '@ngrx/effects/testing';
import { Observable } from 'rxjs';

import { DbusEffects } from './dbus.effects';

describe('DbusEffects', () => {
  let actions$: Observable<any>;
  let effects: DbusEffects;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        DbusEffects,
        provideMockActions(() => actions$)
      ]
    });

    effects = TestBed.inject(DbusEffects);
  });

  it('should be created', () => {
    expect(effects).toBeTruthy();
  });
});
