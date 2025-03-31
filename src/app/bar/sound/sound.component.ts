import { Component, computed, effect } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { fromTauriEvent } from "../../common/tauri-utils";
import { scan } from "rxjs";
import { invoke } from "@tauri-apps/api/core";
import { PipeWireMetadata, PipeWireNode } from "../../common/types";
import { DecimalPipe } from "@angular/common";

@Component({
  selector: "app-sound",
  templateUrl: "./sound.component.html",
  imports: [DecimalPipe],
})
export class SoundComponent {
  readonly defaults = toSignal(
    fromTauriEvent<PipeWireMetadata>("pipewire_metadata"),
  );
  readonly nodes = toSignal(
    fromTauriEvent<PipeWireNode>("pipewire_node").pipe(
      scan(
        (acc, node) => {
          console.log(node);

          acc.nodeMapId.set(node.payload.id, node.payload);
          acc.nodeMapName.set(node.payload.name, node.payload);

          if (acc.nodeMapId.has(node.payload.id)) {
            return {
              ...acc,
              nodes: acc.nodes.map((n) =>
                n.id === node.payload.id ? node.payload : n,
              ),
            };
          } else {
            return {
              ...acc,
              nodes: [...acc.nodes, node.payload],
            };
          }
        },
        {
          nodes: [] as PipeWireNode[],
          nodeMapId: new Map<number, PipeWireNode>(),
          nodeMapName: new Map<string, PipeWireNode>(),
        },
      ),
    ),
  );

  readonly defaultSink = computed(() => {
    const defaultName = this.defaults()?.payload.defaultSink ?? "";
    const sink = this.nodes()?.nodeMapName.get(defaultName);
    return sink;
  });

  readonly defaultVolume = computed(() => {
    return (this.defaultSink()?.volume ?? 0) * 100;
  });

  constructor() {
    effect(() => {
      console.log(this.defaults());
    });

    effect(() => {
      console.log(this.nodes());
    });
  }

  open(): void {
    invoke("open_popup", { popup: "sound" }).then(() => {
      console.log("DAWDAWD");
    });
  }
}
