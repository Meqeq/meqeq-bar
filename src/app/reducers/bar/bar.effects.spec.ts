import { TestBed } from '@angular/core/testing';
import { provideMockActions } from '@ngrx/effects/testing';
import { Observable } from 'rxjs';

import { BarEffects } from './bar.effects';

describe('BarEffects', () => {
  let actions$: Observable<any>;
  let effects: BarEffects;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        BarEffects,
        provideMockActions(() => actions$)
      ]
    });

    effects = TestBed.inject(BarEffects);
  });

  it('should be created', () => {
    expect(effects).toBeTruthy();
  });
});
