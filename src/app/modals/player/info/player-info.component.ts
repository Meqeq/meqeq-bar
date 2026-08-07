import { Component, inject } from '@angular/core';
import { Store } from '@ngrx/store';

import { selectInfoForSelectedPlayer } from '../../../reducers/player/player.selectors';

@Component({
  selector: 'app-player-info',
  templateUrl: './player-info.component.html',
})
export class PlayerInfoComponent {
  private readonly store = inject(Store);

  readonly info = this.store.selectSignal(selectInfoForSelectedPlayer);
}
