import { computed, effect, Injectable } from "@angular/core";
import { fromTauriEvent } from "../common/tauri-utils";
import { map, scan } from "rxjs";
import { PipeWireMetadata, PipeWireNode } from "../common/types";
import { toSignal } from "@angular/core/rxjs-interop";

export interface ActivePopup {
  name: string;
  monitor: number;
}

@Injectable({
  providedIn: "root",
})
export class PopupService {
  readonly activePopup = toSignal(
    fromTauriEvent<ActivePopup>("active_popup").pipe(map((e) => e.payload)),
    { initialValue: { name: "", monitor: 0 } },
  );

  readonly defaults = toSignal(
    fromTauriEvent<PipeWireMetadata>("pipewire_metadata"),
  );
  readonly nodes = toSignal(
    fromTauriEvent<PipeWireNode>("pipewire_node").pipe(
      scan(
        (acc, node) => {
          console.log(node);

          if (acc.nodeMapId.has(node.payload.id)) {
            acc.nodeMapId.set(node.payload.id, node.payload);
            acc.nodeMapName.set(node.payload.name, node.payload);

            return {
              ...acc,
              nodes: acc.nodes.map((n) =>
                n.id === node.payload.id ? node.payload : n,
              ),
            };
          } else {
            acc.nodeMapId.set(node.payload.id, node.payload);
            acc.nodeMapName.set(node.payload.name, node.payload);

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
}
