import { TestBed } from '@angular/core/testing';
import { provideMockActions } from '@ngrx/effects/testing';
import { Observable } from 'rxjs';

import { PipewireEffects } from './pipewire.effects';

describe('PipewireEffects', () => {
  let actions$: Observable<any>;
  let effects: PipewireEffects;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        PipewireEffects,
        provideMockActions(() => actions$)
      ]
    });

    effects = TestBed.inject(PipewireEffects);
  });

  it('should be created', () => {
    expect(effects).toBeTruthy();
  });
});
