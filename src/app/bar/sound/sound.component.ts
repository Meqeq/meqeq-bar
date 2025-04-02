import { DecimalPipe } from "@angular/common";
import { Component, inject } from "@angular/core";
import { PillComponent } from "../../common/pill/pill.component";
import { SoundService } from "../../common/sound.service";

@Component({
  selector: "app-sound",
  templateUrl: "./sound.component.html",
  imports: [DecimalPipe, PillComponent],
})
export class SoundComponent {
  readonly soundService = inject(SoundService);
}
