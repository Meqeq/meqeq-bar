import { computed, signal, Signal } from '@angular/core';
import { toObservable, toSignal } from '@angular/core/rxjs-interop';
import {
  BehaviorSubject,
  Observable,
  Subject,
  switchMap,
  map,
  catchError,
  of,
  partition,
  share,
  tap,
} from 'rxjs';

export interface SimpleDataSourceOptions<RequestType, ItemType> {
  request: () => RequestType;
  sourceFn: (request: RequestType) => Observable<ItemType[]>;
}

export interface SimpleDataSource<ItemType> {
  type: 'simple-data-source';
  items: Signal<ItemType[] | undefined>;

  loading: Signal<boolean>;

  refresh: () => void;
}

export const simpleDataSource = <RequestType, ItemType>(
  options: SimpleDataSourceOptions<RequestType, ItemType>,
): SimpleDataSource<ItemType> => {
  const loading = signal(false);

  const refresh = signal({});
  const request = computed(() => {
    refresh();
    return options.request();
  });

  const result$ = toObservable(request).pipe(
    tap(() => {
      loading.set(true);
    }),
    switchMap((request) => {
      return options.sourceFn(request).pipe(
        map((response) => ({ type: 'ok' as const, response })),
        catchError((error) => {
          return of({
            type: 'error' as const,
            error,
            code: error.code,
          });
        }),
      );
    }),
    share(),
    tap(() => {
      loading.set(false);
    }),
  );

  const [response$, error$] = partition(
    result$,
    (result) => result.type === 'ok',
  );

  const items = toSignal(response$.pipe(map((r) => r.response)));
  const error = toSignal(error$);

  return {
    type: 'simple-data-source',
    items,
    loading,

    refresh: () => {
      refresh.set({});
    },
  };
};
