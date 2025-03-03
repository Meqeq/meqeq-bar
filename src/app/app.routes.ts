import { Routes } from "@angular/router";
import { AppComponent } from "./app.component";

export const routes: Routes = [
  {
    path: "",
    component: AppComponent,
    children: [
      {
        path: "popup/calendar",
        loadComponent: () =>
          import("./popup/calendar/calendar.component").then(
            (c) => c.CalendarComponent,
          ),
      },
      {
        path: "popup/:type",
        loadComponent: () =>
          import("./popup/popup.component").then((c) => c.PopupComponent),
      },
      {
        path: ":monitor",
        loadComponent: () =>
          import("./bar/bar.component").then((c) => c.BarComponent),
      },
    ],
  },
];
