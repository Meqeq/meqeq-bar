import { inject, Injectable, signal } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { toSignal } from '@angular/core/rxjs-interop';
import { map } from 'rxjs';
import { invoke } from '@tauri-apps/api/core';

@Injectable()
export class BarService {
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);

  readonly monitor = toSignal(
    this.route.paramMap.pipe(
      map((paramMap) => Number.parseInt(paramMap.get('monitor') ?? '0')),
    ),
  );

  readonly isPopupOpen = signal(false);

  init() {
    invoke('initialize');
  }

  openPopup(event: MouseEvent, name: string): void {
    event.stopPropagation();

    invoke('set_layer', {
      layer: 'top',
      bar: this.monitor(),
    }).then(() => {
      this.isPopupOpen.set(true);
      this.router.navigate([name], { relativeTo: this.route });
    });
  }

  closePopup(): void {
    if (!this.isPopupOpen()) return;

    this.router.navigate(['./'], { relativeTo: this.route }).then(() => {
      this.isPopupOpen.set(false);
      invoke('set_layer', {
        layer: 'bottom',
        bar: this.monitor(),
      });
    });
  }
}
