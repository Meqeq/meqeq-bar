import { Component, input, OnInit, OnDestroy } from "@angular/core";
import { PipeWireNode } from "../../../common/types";
import { FormControl, ReactiveFormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";
import { debounceTime, Subscription } from "rxjs";

@Component({
  selector: "app-sound-node",
  templateUrl: "./sound-node.component.html",
  imports: [ReactiveFormsModule],
})
export class SoundNodeComponent implements OnInit, OnDestroy {
  readonly node = input.required<PipeWireNode>();
  readonly active = input.required<boolean>();

  readonly volumeControl = new FormControl(0.5);

  private readonly sub = new Subscription();

  ngOnInit(): void {
    this.volumeControl.patchValue(this.node().volume);

    this.volumeControl.valueChanges
      .pipe(debounceTime(100))
      .subscribe((volume) => {
        this.setVolume(this.node().id, volume ?? 0);
      });
  }

  ngOnDestroy(): void {
    this.sub.unsubscribe();
  }

  setDefault(): void {
    invoke("set_default", {
      id: this.node().id,
    });
  }

  setVolume(id: number, volume: number): void {
    console.log(id, volume);
    invoke("set_volume", {
      id,
      volume,
    });
  }
}
