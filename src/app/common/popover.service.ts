import {
  ApplicationRef,
  EmbeddedViewRef,
  inject,
  Injectable,
  Injector,
  Renderer2,
  RendererFactory2,
  TemplateRef,
} from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

@Injectable({
  providedIn: 'root',
})
export class PopoverService {
  private readonly rendererFactory = inject(RendererFactory2);
  private readonly applicationRef = inject(ApplicationRef);
  private readonly injector = inject(Injector);

  private readonly renderer: Renderer2;

  private readonly popoverContainer: HTMLDivElement;

  constructor() {
    this.renderer = this.rendererFactory.createRenderer(null, null);

    const container = this.renderer.selectRootElement('app-bar', true);

    this.popoverContainer = this.renderer.createElement('div');

    this.renderer.setAttribute(
      this.popoverContainer,
      'class',
      'absolute top-0 left-0 w-screen h-screen dropdown dropdown-open overflow-hidden !block',
    );

    this.renderer.appendChild(container, this.popoverContainer);
  }

  open(
    template: TemplateRef<unknown>,
    extra: {
      anchor: HTMLElement;
      context?: object;
      monitor?: number;
      injector?: Injector;
    },
  ): void {
    invoke('set_layer', {
      layer: 'top',
      bar: extra.monitor ?? 0,
    }).then(() => {
      const backdrop = this.createBackdrop();

      const container = this.createContainer();

      const view = template.createEmbeddedView(
        extra.context ?? {},
        extra.injector ? extra.injector : this.injector,
      );

      view.rootNodes.forEach((node) => {
        this.renderer.appendChild(container, node);
      });

      this.applicationRef.attachView(view);
      view.detectChanges();

      this.renderer.appendChild(this.popoverContainer, backdrop);
      this.renderer.appendChild(this.popoverContainer, container);

      const [posX, posY] = this.getPosition(extra.anchor, 'tl');

      console.log(
        posX,
        posY,
        extra.anchor.clientTop,
        extra.anchor.offsetTop,
        extra.anchor.scrollTop,
        container.getBoundingClientRect(),
      );

      this.renderer.setStyle(container, 'top', `${posY}px`);
      this.renderer.setStyle(container, 'left', `${posX}px`);

      backdrop.addEventListener('click', () => {
        this.renderer.removeChild(this.popoverContainer, container);
        this.renderer.removeChild(this.popoverContainer, backdrop);
        view.destroy();
        invoke('set_layer', {
          layer: 'bottom',
          bar: extra.monitor ?? 0,
        });
      });

      container.addEventListener('mouseleave', () => {
        this.renderer.removeChild(this.popoverContainer, container);
        this.renderer.removeChild(this.popoverContainer, backdrop);
        view.destroy();

        invoke('set_layer', {
          layer: 'bottom',
          bar: extra.monitor ?? 0,
        });
      });
    });
  }

  private createBackdrop(): HTMLDivElement {
    const backdrop = this.renderer.createElement('div');
    this.renderer.setAttribute(
      backdrop,
      'class',
      'w-full h-full pointer-events-auto absolute top-0 left-0',
    );
    return backdrop;
  }

  private createContainer(): HTMLDivElement {
    const container = this.renderer.createElement('div');
    this.renderer.setAttribute(
      container,
      'class',
      'pointer-events-auto absolute dropdown-content !block',
    );

    return container;
  }

  private getPosition(
    anchor: HTMLElement,
    corner: 'tl' | 'tr' | 'bl' | 'br',
  ): [number, number] {
    const rec = anchor.getBoundingClientRect();

    switch (corner) {
      case 'tl':
        return [rec.left, rec.top];
      case 'tr':
        return [rec.right, rec.top];
      case 'bl':
        return [rec.left, rec.bottom];
      case 'br':
        return [rec.right, rec.bottom];
    }
  }
}
