import { Component, inject, input } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { ActivatedRoute } from "@angular/router";

@Component({
  standalone: true,
  selector: "app-popup",
  templateUrl: "./popup.component.html",
})
export class PopupComponent {
  private readonly route = inject(ActivatedRoute);

  readonly type = input.required<string>();

  ngOnInit(): void {
    this.route.paramMap.subscribe(console.log);
  }
}
