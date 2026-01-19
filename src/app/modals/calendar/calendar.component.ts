import { DatePipe } from '@angular/common';
import {
  Component,
  computed,
  effect,
  ElementRef,
  viewChild,
} from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import Pikaday from 'pikaday';
import { interval, map, startWith } from 'rxjs';

@Component({
  selector: 'app-calendar-modal',
  templateUrl: './calendar.component.html',
  styleUrl: './calendar.component.scss',
  imports: [DatePipe],
})
export class CalendarModalComponent {
  readonly time = toSignal(
    interval(1000).pipe(
      startWith(null),
      map(() => new Date()),
    ),
  );

  readonly calendarRef =
    viewChild.required<ElementRef<HTMLInputElement>>('calendar');
  readonly inputRef = viewChild.required<ElementRef<HTMLInputElement>>('input');

  readonly picker = computed(() => {
    return new Pikaday({
      field: this.inputRef().nativeElement,
      container: this.calendarRef().nativeElement,
      firstDay: 1,
      bound: false,
      i18n: {
        previousMonth: 'Poprzedni miesiąc',
        nextMonth: 'Następny miesiąc',
        months: [
          'Styczeń',
          'Luty',
          'Marzec',
          'Kwiecień',
          'Maj',
          'Czerwiec',
          'Lipiec',
          'Sierpień',
          'Wrzesień',
          'Październik',
          'Listopad',
          'Grudzień',
        ],
        weekdays: [
          'Niedziela',
          'Poniedziałek',
          'Wtorek',
          'Środa',
          'Czwartek',
          'Piątek',
          'Sobota',
        ],
        weekdaysShort: ['Nie', 'Pon', 'Wto', 'Śro', 'Czw', 'Pią', 'Sob'],
      },
    });
  });

  constructor() {
    effect(() => {
      if (this.inputRef() && this.calendarRef()) this.picker().show();
    });
  }
}
