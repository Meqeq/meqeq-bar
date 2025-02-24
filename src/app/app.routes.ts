import { Routes } from "@angular/router";
import { KekComponent } from "./kek/kek.component";

export const routes: Routes = [
  {
    path: ":kek",
    component: KekComponent,
  },
];
