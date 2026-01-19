import {
  computed,
  effect,
  signal,
  Signal,
  WritableSignal,
} from '@angular/core';
import { toObservable, toSignal } from '@angular/core/rxjs-interop';
import {
  catchError,
  map,
  Observable,
  of,
  partition,
  share,
  switchMap,
} from 'rxjs';

export type SortDirection = 'asc' | 'desc';

export interface SortConfig {
  key: string;
  direction: SortDirection;
}

export type FilterType = 'single' | 'multi';

export interface Filter {
  key: string;
  type: FilterType;
  value: unknown;
}

export type LoadingState = 'skeleton' | 'loader' | 'search' | false;

export type LoadingReason =
  | 'pagination'
  | 'search'
  | 'filter'
  | 'sort'
  | 'refresh'
  | undefined;

export interface DataSource<ItemType> {
  type: 'data-source';
  page: WritableSignal<number>;
  size: WritableSignal<number>;
  search: WritableSignal<string>;

  items: Signal<ItemType[]>;
  itemCount: Signal<number>;
  pageCount: Signal<number>;

  sort: Signal<SortConfig[]>;
  filters: Signal<Filter[]>;

  loading: Signal<LoadingState>;
  loadingReason: Signal<LoadingReason>;

  response$: Observable<PaginatedResponse<ItemType>>;

  setSort: (key: string, direction?: SortDirection) => void;
  setFilter: (filter: Filter) => void;
  clearSort: () => void;
  clearFilters: () => void;
  refresh: () => void;
}

export interface PaginatedRequest {
  page: number;
  size: number;
  search: string;

  sort: SortConfig[];
  filters: Filter[];
}

export interface PaginatedResponse<ItemType> {
  items: ItemType[];
  page: number;
  size: number;
  pageCount: number;
  itemCount: number;
}

export type PaginatedResult<ItemType> =
  | {
      type: 'ok';
      response: PaginatedResponse<ItemType>;
    }
  | {
      type: 'error';
      code: number;
      error: any;
    };

export type SourceFn<ItemType> = (
  request: PaginatedRequest,
) => Observable<PaginatedResponse<ItemType>>;

export interface DataSourceOptions {
  filters?: Filter[] | (() => Filter[]);
}

export const dataSource = <ItemType>(
  sourceFn: SourceFn<ItemType>,
  options?: DataSourceOptions,
): DataSource<ItemType> => {
  const sortMap = new Map<string, SortDirection>();
  const filterMap = new Map<string, Filter>();

  const page = signal(0);
  const size = signal(50);
  const search = signal('');

  const sort = signal<SortConfig[]>([]);
  const filters = signal<Filter[]>([]);

  const loading = signal<LoadingState>('skeleton');
  const loadingReason = signal<LoadingReason>(undefined);

  const setFilter = (filter: Filter) => {
    filterMap.set(filter.key, filter);

    filters.set([...filterMap.values()]);
  };

  if (options?.filters) {
    if (typeof options.filters === 'function') {
      const fn = options.filters;
      effect(() => {
        fn().forEach((filter) => setFilter(filter));
      });
    } else {
      options.filters.forEach((filter) => setFilter(filter));
    }
  }

  effect(() => {
    page();
    loadingReason.set('pagination');
  });

  effect(() => {
    search();
    loadingReason.set('search');
  });

  effect(() => {
    filters();
    loadingReason.set('filter');
  });

  const request = computed((): PaginatedRequest => {
    switch (loadingReason()) {
      case 'search':
      case 'filter':
        return {
          page: 0,
          size: size(),
          sort: sort(),
          search: search(),
          filters: filters(),
        };

      default:
        return {
          page: page(),
          size: size(),
          sort: sort(),
          search: search(),
          filters: filters(),
        };
    }
  });

  const result$ = toObservable(request).pipe(
    switchMap((request): Observable<PaginatedResult<ItemType>> => {
      return sourceFn(request).pipe(
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
  );

  const [response$, error$] = partition(
    result$,
    (result) => result.type === 'ok',
  );

  const response = toSignal(response$.pipe(map((r) => r.response)));
  const error = toSignal(error$);

  return {
    type: 'data-source',
    page,
    size,
    search,

    sort,
    filters,

    response$: response$.pipe(map((r) => r.response)),

    loading,
    loadingReason,

    items: computed(() => response()?.items ?? []),
    itemCount: computed(() => response()?.itemCount ?? 0),
    pageCount: computed(() => response()?.pageCount ?? 0),

    setSort: (key: string) => {
      const config = sortMap.get(key);

      if (config === 'asc') sortMap.set(key, 'desc');
      else if (config === 'desc') sortMap.delete(key);
      else sortMap.set(key, 'asc');

      sort.set(
        [...sortMap.entries()].map((entry) => ({
          key: entry[0],
          direction: entry[1],
        })),
      );
    },
    setFilter,
    clearSort: () => {},
    clearFilters: () => {},
    refresh: () => {
      sort.update((s) => [...s]);
    },
  };
};
