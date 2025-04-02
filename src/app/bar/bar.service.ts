import { inject, Injectable, signal } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { ActivatedRoute, Router } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { map } from "rxjs";

@Injectable()
export class BarService {
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);

  readonly monitor = toSignal(
    this.route.paramMap.pipe(
      map((paramMap) => Number.parseInt(paramMap.get("monitor") ?? "0")),
    ),
    { initialValue: 0 },
  );

  private readonly isPopupOpen = signal(false);

  init(): void {
    invoke("initialize");
  }

  openPopup(event: MouseEvent, name: string): void {
    event.stopPropagation();

    invoke("set_layer", {
      layer: "top",
      bar: this.monitor(),
    }).then(() => {
      this.isPopupOpen.set(true);
      this.router.navigate([name], { relativeTo: this.route });
    });
  }

  closePopup(): void {
    if (!this.isPopupOpen()) return;

    this.router.navigate(["./"], { relativeTo: this.route }).then(() => {
      invoke("set_layer", {
        layer: "bottom",
        bar: this.monitor(),
      });
    });
  }
}
