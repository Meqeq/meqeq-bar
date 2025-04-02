import { Component, computed, inject } from "@angular/core";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";

import { JsonPipe } from "@angular/common";
import { SoundService } from "../../common/sound.service";
import { PipeWireNode } from "../../common/types";
import { SoundNodeComponent } from "./node/sound-node.component";

@Component({
  selector: "app-sound-popup",
  templateUrl: "./sound-popup.component.html",
  imports: [JsonPipe, FormsModule, ReactiveFormsModule, SoundNodeComponent],
})
export class SoundPopupComponent {
  readonly soundService = inject(SoundService);

  readonly data = computed(() => {
    const sinks: PipeWireNode[] = [];
    const sources: PipeWireNode[] = [];

    this.soundService.nodes()?.nodes.forEach((node) => {
      switch (node.class) {
        case "Audio/Sink":
          sinks.push(node);
          break;
        case "Audio/Source":
          sources.push(node);
          break;
      }
    });

    return {
      sinks,
      sources,
    };
  });

  setVolume(id: number, value: number): void {
    console.log(id, value);
    invoke("set_volume", {
      id,
      value,
    }).then(() => {
      console.log("DAWDAWD");
    });
  }
}
