import { AsyncPipe } from "@angular/common";
import { Component, inject } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { map } from "rxjs";

@Component({
  standalone: true,
  selector: "app-kek",
  templateUrl: "./kek.component.html",
  imports: [AsyncPipe],
})
export class KekComponent {
  private readonly route = inject(ActivatedRoute);

  readonly kek$ = this.route.paramMap.pipe(
    map((paramMap) => paramMap.get("kek")),
  );
}
