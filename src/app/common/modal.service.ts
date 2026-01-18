import {
  ApplicationRef,
  ComponentRef,
  EnvironmentInjector,
  Injectable,
  InjectionToken,
  Injector,
  Provider,
  Renderer2,
  RendererFactory2,
  Type,
  createComponent,
  inject,
} from '@angular/core';
import { Store } from '@ngrx/store';
import { selectRouteParam } from '../reducers/router/router.selectors';
import { invoke } from '@tauri-apps/api/core';
import { BarActions } from '../reducers/bar/bar.actions';

export interface ModalInstance {
  resolve: (value: unknown) => void;
  reject: (error: unknown) => void;
}

export const ModalPosition = {
  TopLeft: 'tl',
  TopRight: 'tr',
  BottomLeft: 'bl',
  BottomRight: 'br',
  Center: 'c',
} as const;

export type ModalPosition = (typeof ModalPosition)[keyof typeof ModalPosition];

export interface ModalOptions {
  withBackdrop?: true;
  allowMultiple?: true;
  size?: string;
  position?: ModalPosition;
  context?: unknown;
  providers?: Provider[];
}

export const MODAL_INSTANCE = new InjectionToken<ModalInstance>(
  'MODAL_SERVICE',
);
export const MODAL_CONTEXT = new InjectionToken<unknown>('MODAL_CONTEXT');

interface ModalElements {
  dialog: HTMLDialogElement;
  dialogBox: HTMLDivElement;
  backdrop: HTMLDivElement;
}

@Injectable({
  providedIn: 'root',
})
export class ModalService {
  private readonly environmentInjector = inject(EnvironmentInjector);
  private readonly rendererFactory = inject(RendererFactory2);
  private readonly applicationRef = inject(ApplicationRef);
  private readonly injector = inject(Injector);
  private readonly store = inject(Store);

  private readonly renderer: Renderer2;
  private readonly container: HTMLBodyElement;
  private readonly monitor = this.store.selectSignal(
    selectRouteParam('monitor'),
  );

  private readonly modals = new Map<Type<unknown>, ModalInstance>();

  constructor() {
    this.renderer = this.rendererFactory.createRenderer(null, null);
    this.container = this.renderer.selectRootElement('app-root', true);
  }

  open<ReturnType>(
    component: Type<unknown>,
    options?: ModalOptions,
  ): Promise<ReturnType> {
    if (options?.allowMultiple !== true && this.modals.has(component)) {
      this.modals.get(component)?.resolve(false);
      return new Promise((resolve) => resolve(false as ReturnType));
    }

    const { dialog, dialogBox, backdrop } = this.createElements(
      options?.position ?? ModalPosition.BottomRight,
      options?.size ?? 'w-128 h-64',
    );

    const instance: ModalInstance = {
      resolve: (_value: unknown) => {},
      reject: (_error: unknown) => {},
    };

    const injector = this.createInjector(
      instance,
      options?.context,
      options?.providers,
    );

    const modalRef = this.createModalRef(component, dialogBox, injector);

    backdrop.addEventListener('click', () => {
      instance.resolve(false);
    });

    invoke('set_layer', {
      layer: 'top',
      bar: Number.parseInt(this.monitor() ?? '0', 10),
    }).then(() => {
      this.modals.set(component, instance);

      if (options?.withBackdrop) {
        dialog.showModal();
      } else {
        dialog.show();
      }
    });

    return new Promise((resolve, reject) => {
      instance.resolve = (value: unknown) => {
        dialog.close();
        resolve(value as ReturnType);
        this.modals.delete(component);

        setTimeout(() => {
          modalRef.destroy();
          this.renderer.removeChild(this.container, dialog);
          this.store.dispatch(BarActions.setBottomLayer());
        }, 300);
      };

      instance.reject = (error: unknown) => {
        dialog.close();
        reject(error);
        this.modals.delete(component);

        setTimeout(() => {
          modalRef.destroy();
          this.renderer.removeChild(this.container, dialog);
          this.store.dispatch(BarActions.setBottomLayer());
        }, 300);
      };
    });
  }

  private createInjector(
    instance: ModalInstance,
    context: unknown,
    providers?: Provider[],
  ): Injector {
    const elementInjector = Injector.create({
      providers: [
        ...(providers ?? []),
        { provide: MODAL_INSTANCE, useValue: instance },
        { provide: MODAL_CONTEXT, useValue: context },
      ],
      parent: this.injector,
    });

    return elementInjector;
  }

  private createModalRef(
    component: Type<unknown>,
    hostElement: HTMLDivElement,
    elementInjector: Injector,
  ): ComponentRef<unknown> {
    const componentRef = createComponent(component, {
      environmentInjector: this.environmentInjector,
      elementInjector,
      hostElement,
    });

    this.applicationRef.attachView(componentRef.hostView);

    return componentRef;
  }

  private createElements(position: ModalPosition, size: string): ModalElements {
    const dialog: HTMLDialogElement = this.renderer.createElement('dialog');

    this.renderer.setAttribute(
      dialog,
      'class',
      `modal p-2 pb-12 ${this.getPositionClasses(position)}`,
    );

    const dialogBox = this.renderer.createElement('div');
    this.renderer.setAttribute(
      dialogBox,
      'class',
      `modal-box w-max h-max rounded-box max-w-none`,
    );

    this.renderer.appendChild(this.container, dialog);
    this.renderer.appendChild(dialog, dialogBox);

    const backdrop: HTMLDivElement = this.renderer.createElement('div');
    this.renderer.setAttribute(backdrop, 'class', 'modal-backdrop');
    this.renderer.appendChild(dialog, backdrop);

    return {
      dialog,
      dialogBox,
      backdrop,
    };
  }

  private getPositionClasses(position: ModalPosition): string {
    switch (position) {
      case ModalPosition.TopLeft:
        return 'modal-start modal-top';
      case ModalPosition.TopRight:
        return 'modal-end modal-top';
      case ModalPosition.BottomLeft:
        return 'modal-start modal-bottom';
      case ModalPosition.BottomRight:
        return 'modal-end modal-bottom';
      case ModalPosition.Center:
        return '';
    }
  }
}
