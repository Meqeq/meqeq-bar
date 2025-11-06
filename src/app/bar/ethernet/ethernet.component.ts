import { Component } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import {
  Cable,
  EthernetPort,
  LucideAngularModule,
  MonitorDot,
  MonitorSmartphone,
  ScreenShareOff,
  Unplug,
} from 'lucide-angular';
import { fromEvent, map, merge, startWith } from 'rxjs';

@Component({
  selector: 'app-ethernet',
  templateUrl: './ethernet.component.html',
  imports: [LucideAngularModule],
})
export class EthernetComponent {
  readonly ethernetIcon = EthernetPort;

  readonly status$ = merge(
    fromEvent(window, 'online').pipe(map(() => 'online' as const)),
    fromEvent(window, 'offline').pipe(map(() => 'offline' as const)),
  ).pipe(startWith(navigator.onLine ? 'online' : 'offline'));

  readonly icon = toSignal(
    this.status$.pipe(
      map((status) =>
        status === 'online' ? MonitorSmartphone : ScreenShareOff,
      ),
    ),
  );
}
