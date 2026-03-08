import {
  ApplicationRef,
  DestroyRef,
  EmbeddedViewRef,
  inject,
  Injectable,
  Injector,
  Renderer2,
  RendererFactory2,
  TemplateRef,
  ElementRef,
  computed,
  Signal,
} from '@angular/core';

import { toObservable, toSignal } from '@angular/core/rxjs-interop';
import { fromEvent, switchMap } from 'rxjs';
import { map } from 'rxjs/operators';

export interface Popover {
  id: Signal<string>;
  isOpen: Signal<boolean>;
  open: () => Promise<void>;
  close: () => void;
}

interface CreatedPopover {
  popover: HTMLDivElement;
  viewRef: EmbeddedViewRef<unknown>;
}

interface PopoverOptions {
  setMinWidth?: true;
}

export const popover = (
  popoverTemplate: Signal<TemplateRef<unknown>>,
  anchor: Signal<ElementRef<HTMLElement>>,
  options?: PopoverOptions,
): Popover => {
  const popoverService = inject(PopoverService);
  const destroyRef = inject(DestroyRef);
  const injector = inject(Injector);

  const created = computed(() => {
    return popoverService.create(popoverTemplate(), {
      anchor: anchor().nativeElement,
      injector,
      useMinWidth: options?.setMinWidth,
    });
  });

  destroyRef.onDestroy(() => {
    popoverService.destroy(created());
  });

  const isOpen = toObservable(created).pipe(
    switchMap((pop: CreatedPopover) => {
      return fromEvent(pop.popover, 'toggle').pipe(
        map((e) => (e as ToggleEvent).newState === 'open'),
      );
    }),
  );

  return {
    id: computed(() => created().popover.id),
    isOpen: toSignal(isOpen, { initialValue: false }),
    open: () =>
      new Promise((resolve) => {
        (created().popover as any).showPopover({
          source: anchor().nativeElement,
        });

        const onToggle = (e: ToggleEvent) => {
          if (e.newState === 'closed') {
            created().popover.removeEventListener('toggle', onToggle);
            resolve();
          }
        };

        created().popover.addEventListener('toggle', onToggle);
      }),
    close: () => {
      created().popover.togglePopover(false);
    },
  };
};

@Injectable({
  providedIn: 'root',
})
export class PopoverService {
  private readonly rendererFactory = inject(RendererFactory2);
  private readonly applicationRef = inject(ApplicationRef);
  private readonly injector = inject(Injector);

  private readonly renderer: Renderer2;
  private readonly container: HTMLDivElement;

  private popoverNumber = 1;

  constructor() {
    this.renderer = this.rendererFactory.createRenderer(null, null);
    this.container = this.renderer.selectRootElement('app-root', true);
  }

  create(
    template: TemplateRef<unknown>,
    extra: {
      anchor: HTMLElement;
      context?: object;
      injector?: Injector;
      useMinWidth?: true;
    },
  ): CreatedPopover {
    const viewRef = template.createEmbeddedView(
      extra.context ?? {},
      extra.injector ? extra.injector : this.injector,
    );

    const popover: HTMLDivElement = this.renderer.createElement('div');

    this.renderer.setAttribute(popover, 'popover', '');
    this.renderer.setAttribute(
      popover,
      'id',
      `mqq-popover-${this.popoverNumber}`,
    );
    this.renderer.setAttribute(
      popover,
      'class',
      'dropdown bg-base-100 shadow-lg rounded-box',
    );

    // this.renderer.setAttribute(
    //   extra.anchor,
    //   'popovertarget',
    //   `mqq-popover-${this.popoverNumber}`,
    // );

    this.renderer.setStyle(
      extra.anchor,
      'anchor-name',
      `--mqq-popover-anchor-${this.popoverNumber}`,
    );

    this.renderer.setStyle(
      popover,
      'position-anchor',
      `--mqq-popover-anchor-${this.popoverNumber}`,
    );

    if (extra.useMinWidth)
      this.renderer.setStyle(
        popover,
        'min-width',
        `${extra.anchor.clientWidth}px`,
      );

    this.renderer.appendChild(this.container, popover);

    this.renderer.setStyle(popover, 'position-area', 'bottom span-right');

    this.applicationRef.attachView(viewRef);

    viewRef.rootNodes.forEach((node) => {
      this.renderer.appendChild(popover, node);
    });

    this.popoverNumber += 1;

    return {
      popover,
      viewRef,
    };
  }

  destroy(created: CreatedPopover): void {
    created.viewRef.destroy();
    this.renderer.removeChild(this.container, created.popover);
  }
}
