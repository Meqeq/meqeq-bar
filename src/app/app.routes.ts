import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: '',
    children: [
      {
        path: 'bar/:monitor',
        loadComponent: () =>
          import('./bar/bar.component').then((c) => c.BarComponent),
        children: [
          {
            path: 'calendar',
            loadComponent: () =>
              import('./calendar/calendar.component').then(
                (c) => c.CalendarComponent,
              ),
          },
          {
            path: 'sound',
            loadComponent: () =>
              import('./sound-new/sound-new.component').then(
                (c) => c.SoundNewComponent,
              ),
          },
        ],
      },
    ],
  },
];
