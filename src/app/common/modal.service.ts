import {
  ApplicationRef,
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

export interface ModalInstance {
  resolve: (value: unknown) => {};
  reject: (error: unknown) => {};
}

export interface ModalOptions {
  context?: unknown;
  providers?: Provider[];
}

export const MODAL_INSTANCE = new InjectionToken<ModalInstance>(
  'MODAL_SERVICE',
);
export const MODAL_CONTEXT = new InjectionToken<unknown>('MODAL_CONTEXT');

@Injectable({
  providedIn: 'root',
})
export class ModalService {
  private readonly environmentInjector = inject(EnvironmentInjector);
  private readonly rendererFactory = inject(RendererFactory2);
  private readonly applicationRef = inject(ApplicationRef);
  private readonly injector = inject(Injector);

  private readonly renderer: Renderer2;
  private readonly container: HTMLBodyElement;

  constructor() {
    this.renderer = this.rendererFactory.createRenderer(null, null);

    this.container = this.renderer.selectRootElement('app-root', true);
  }

  open(component: Type<unknown>) {
    this.createModal(component);
  }

  private createModal(component: Type<unknown>) {
    const dialog: HTMLDialogElement = this.renderer.createElement('dialog');

    this.renderer.setAttribute(dialog, 'class', 'modal modal-end modal-bottom');

    const dialogBox = this.renderer.createElement('div');
    this.renderer.setAttribute(dialogBox, 'class', 'modal-box h-120');

    this.createModalComponent(component, dialogBox);
    this.renderer.appendChild(this.container, dialog);
    this.renderer.appendChild(dialog, dialogBox);
    // dialog.showModal();
    dialog.show();
  }

  private createModalComponent(
    component: Type<unknown>,
    hostElement: HTMLDivElement,
    options?: ModalOptions,
  ) {
    const instance = {
      resolve: (value: unknown) => {},
      reject: (error: unknown) => {},
    };

    const elementInjector = Injector.create({
      providers: [
        ...(options?.providers ?? []),
        { provide: MODAL_INSTANCE, useValue: instance },
        { provide: MODAL_CONTEXT, useValue: options?.context },
      ],
      parent: this.injector,
    });

    const componentRef = createComponent(component, {
      environmentInjector: this.environmentInjector,
      elementInjector,
      hostElement,
    });
    console.log(componentRef);
    this.applicationRef.attachView(componentRef.hostView);

    instance.resolve = (value: unknown) => {
      componentRef.destroy();
      // resolve(value);
    };

    instance.reject = (error: unknown) => {
      // hostElement.close();
      componentRef.destroy();
      // reject(error);
    };

    return componentRef;
  }
}
