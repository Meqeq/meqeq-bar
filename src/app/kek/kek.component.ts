import { Component, inject } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { map } from "rxjs";
import { invoke } from "@tauri-apps/api/core";

@Component({
  standalone: true,
  selector: "app-kek",
  templateUrl: "./kek.component.html",
})
export class KekComponent {
  private readonly route = inject(ActivatedRoute);

  readonly kek$ = this.route.paramMap.pipe(
    map((paramMap) => paramMap.get("kek")),
  );

  ngOnInit(): void {
    setTimeout(() => {
      invoke("on_workspace_add").then(() => {});
      invoke("on_workspace_remove").then(() => {});
      invoke("on_active_window_change");
    }, 1000);
  }
}
