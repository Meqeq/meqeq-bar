import {
  ChangeDetectorRef,
  Component,
  effect,
  ElementRef,
  inject,
  InjectionToken,
  input,
  model,
  output,
  signal,
  TemplateRef,
  untracked,
  viewChild,
} from '@angular/core';
import {
  ControlValueAccessor,
  FormsModule,
  NG_VALUE_ACCESSOR,
} from '@angular/forms';
import { LucideAngularModule, Search } from 'lucide-angular';
import { DataSource } from '../data-source';
import { debounceTime, fromEvent, scan, Subscription } from 'rxjs';
import { toObservable, toSignal } from '@angular/core/rxjs-interop';
import { NgTemplateOutlet } from '@angular/common';
import { SimpleDataSource } from '../simple-data-source';

export interface ComboConfig {
  size: number;
  cachedPages: number;
}

export const COMBO_CONFIG = new InjectionToken<ComboConfig>('COMBO_CONFIG', {
  factory: () => {
    return {
      size: 50,
      cachedPages: 3,
    };
  },
});

@Component({
  selector: 'app-combo',
  templateUrl: './combo.component.html',
  imports: [FormsModule, LucideAngularModule, NgTemplateOutlet],
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      multi: true,
      useExisting: ComboComponent,
    },
  ],
})
export class ComboComponent<ItemType extends any>
  implements ControlValueAccessor
{
  private readonly cdr = inject(ChangeDetectorRef);
  private readonly config = inject(COMBO_CONFIG);

  readonly name = input('');
  readonly label = input('');
  readonly rowTemplate = input<TemplateRef<unknown>>();

  readonly search = model('');

  readonly valueKey = input<any>('id');
  readonly displayKey = input<any>('name');

  readonly dataSource = input<
    DataSource<ItemType> | SimpleDataSource<ItemType>
  >();

  readonly value = signal('');
  readonly selected = model<ItemType>(undefined as ItemType);
  readonly disabled = signal(false);

  readonly actionButton = input('');

  readonly actionButtonClick = output();

  readonly scrollContainer =
    viewChild.required<ElementRef<HTMLElement>>('scrollContainer');

  readonly firstItem = viewChild<ElementRef<HTMLElement>>('item');

  readonly currentScroll = signal({
    scrollTop: 0,
    scrollHeight: 1000,
    clientHeight: 100,
  });

  readonly items = model<ItemType[]>([]);
  readonly page = signal(-1);
  readonly pageCount = signal(0);

  readonly debouncedSearch = toSignal(
    toObservable(this.search).pipe(debounceTime(300)),
    { initialValue: '' },
  );

  constructor() {
    effect((onCleanup) => {
      const sub = fromEvent(
        this.scrollContainer().nativeElement,
        'scroll',
      ).subscribe((event) => {
        const target = event.currentTarget as HTMLElement;

        const { scrollTop, scrollHeight, clientHeight } = target;

        this.currentScroll.set({
          scrollTop,
          scrollHeight,
          clientHeight,
        });
      });

      console.log('BBBB');

      onCleanup(() => {
        sub.unsubscribe();
      });
    });

    effect((onCleanup) => {
      const dataSource = this.dataSource();
      const sub = new Subscription();

      console.log('CCCCC');
      if (!dataSource) return;

      if (dataSource.type === 'data-source') {
        sub.add(
          dataSource.response$
            .pipe(
              scan(
                (acc, current) => {
                  const reason = untracked(() => dataSource.loadingReason());

                  switch (reason) {
                    case 'pagination':
                      const direction = current.page - acc.page;

                      return {
                        items: this.getItems(
                          acc.items,
                          current.items,
                          direction,
                        ),
                        page: direction > 0 ? current.page : acc.page - 1,
                        pageCount: current.pageCount,
                      };

                    default:
                      return {
                        items: current.items,
                        page: current.page,
                        pageCount: current.pageCount,
                      };
                  }
                },
                {
                  items: [],
                  page: -1,
                  pageCount: 0,
                } as any,
              ),
            )
            .subscribe((res) => {
              const firstItem = this.firstItem();
              const hasToScroll = res.page < this.page();

              this.items.set(res.items);
              this.page.set(res.page);
              this.pageCount.set(res.pageCount);

              if (firstItem && hasToScroll) {
                untracked(() => {
                  this.cdr.detectChanges();

                  this.scrollContainer().nativeElement.scrollTo({
                    top: firstItem.nativeElement.offsetTop,
                  });
                });
              }
            }),
        );
      } else {
        const items = dataSource.items();
        if (items) this.items.set(items);
      }

      onCleanup(() => {
        sub.unsubscribe();
      });
    });

    effect(() => {
      const dataSource = this.dataSource();
      console.log('DDDDD');
      if (!dataSource) return;

      if (
        dataSource.type === 'data-source' &&
        this.currentScroll().scrollHeight ===
          this.currentScroll().scrollTop + this.currentScroll().clientHeight
      ) {
        untracked(() => {
          if (this.page() < this.pageCount())
            dataSource.page.set(this.page() + 1);
        });
      }

      if (
        dataSource.type === 'data-source' &&
        this.currentScroll().scrollTop === 0
      ) {
        untracked(() => {
          if (this.page() >= this.config.cachedPages)
            dataSource.page.set(this.page() - this.config.cachedPages);
        });
      }
    });

    effect(() => {
      const dataSource = this.dataSource();
      if (!dataSource) return;

      if (dataSource.type === 'data-source')
        dataSource.search.set(this.debouncedSearch());
    });
  }

  ngOnInit(): void {
    if (this.value() && !this.selected()) {
      const kek = this.items().find(
        (item) => item[this.valueKey() as keyof typeof item] === this.value(),
      );

      if (kek) this.handleSelect(kek);
    }
  }

  scrollToTop(): void {
    // this.scrollContainer().nativeElement.scrollTo({
    //   top: 0,
    // });
  }

  handleSelect(item: ItemType): void {
    this.value.set((item as any).id);
    this.selected.set(item);

    this.onChange(this.value());

    this.scrollContainer().nativeElement.parentElement?.parentElement?.focus();
  }

  private getItems(
    accumulated: ItemType[],
    newItems: ItemType[],
    direction: number,
  ): ItemType[] {
    const slice =
      accumulated.length >
      (this.config.cachedPages - 1) * (this.dataSource() as any).size();

    if (direction > 0) {
      if (slice) {
        return [
          ...accumulated.slice((this.dataSource() as any).size()),
          ...newItems,
        ];
      }

      return [...accumulated, ...newItems];
    }
    console.log('PRE');
    return [
      ...newItems,
      ...accumulated.slice(
        0,
        (this.config.cachedPages - 1) * (this.dataSource() as any).size(),
      ),
    ];
  }

  onChange = (_value: string) => {};

  onTouched = () => {};

  writeValue(value: string): void {
    this.value.set(value);

    const kek = this.items().find(
      (item) => item[this.valueKey() as keyof typeof item] === value,
    );

    if (kek) this.handleSelect(kek);
  }

  registerOnChange(fn: (value: string) => void): void {
    this.onChange = fn;
  }

  registerOnTouched(fn: () => void): void {
    this.onTouched = fn;
  }

  setDisabledState(isDisabled: boolean): void {
    this.disabled.set(isDisabled);
  }

  readonly searchIcon = Search;
}
