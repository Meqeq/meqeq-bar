import { TestBed } from '@angular/core/testing';
import { provideMockActions } from '@ngrx/effects/testing';
import { Observable } from 'rxjs';

import { HyprlandEffects } from './hyprland.effects';

describe('HyprlandEffects', () => {
  let actions$: Observable<any>;
  let effects: HyprlandEffects;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        HyprlandEffects,
        provideMockActions(() => actions$)
      ]
    });

    effects = TestBed.inject(HyprlandEffects);
  });

  it('should be created', () => {
    expect(effects).toBeTruthy();
  });
});
