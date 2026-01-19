import { Component, computed, effect, inject, signal } from '@angular/core';

import {
  LucideAngularModule,
  Mic,
  MicOff,
  Volume,
  Volume1,
  Volume2,
  VolumeOff,
} from 'lucide-angular';
import { DecimalPipe } from '@angular/common';
import { Store } from '@ngrx/store';
import {
  selectDefaultSink,
  selectDefaultSource,
  selectIsRecordingActive,
} from '../../reducers/pipewire/pipewire.selectors';
import { toObservable, toSignal } from '@angular/core/rxjs-interop';
import { delay, map, switchMap } from 'rxjs/operators';
import { merge, of } from 'rxjs';
import { PipewireActions } from '../../reducers/pipewire/pipewire.actions';
import { PwRouteDirection } from '../../reducers/pipewire/pipewire.schema';

@Component({
  selector: 'app-sound',
  templateUrl: './sound.component.html',
  imports: [LucideAngularModule, DecimalPipe],
})
export class SoundComponent {
  private readonly store = inject(Store);

  readonly defaultSinkDevice = this.store.selectSignal(selectDefaultSink);
  readonly defaultSourceDevice = this.store.selectSignal(selectDefaultSource);
  readonly isRecordingActive = this.store.selectSignal(selectIsRecordingActive);

  readonly volume = computed(() => {
    return Math.round(
      (this.defaultSinkDevice()?.route.output?.volume[0] ?? 0) * 100,
    );
  });

  readonly sinkIcon = computed(() => {
    if (this.defaultSinkDevice()?.route?.output?.mute || this.volume() < 1)
      return VolumeOff;

    if (this.volume() > 50) return Volume2;

    if (this.volume() > 10) return Volume1;

    return Volume;
  });

  changeSourceMute(): void {
    const source = this.defaultSourceDevice();

    if (!source) return;

    const props = {
      id: source.id,
      routeType: 'input' as const,
    };

    this.store.dispatch(
      source.route.input?.mute
        ? PipewireActions.unmuteDevice(props)
        : PipewireActions.muteDevice(props),
    );
  }

  private readonly volume$ = toObservable(this.volume);
  readonly highlight = toSignal(
    merge(
      this.volume$.pipe(map(() => true)),
      this.volume$.pipe(switchMap(() => of(false).pipe(delay(1000)))),
    ),
    { initialValue: false },
  );

  readonly micIcon = computed(() => {
    if (this.defaultSourceDevice()?.route.input?.mute) return MicOff;
    return Mic;
  });
}
