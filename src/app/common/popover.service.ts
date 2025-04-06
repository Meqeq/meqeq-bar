import {
  ApplicationRef,
  EmbeddedViewRef,
  inject,
  Injectable,
  Injector,
  Renderer2,
  RendererFactory2,
  signal,
  TemplateRef,
} from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { ActivatedRoute } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { map } from "rxjs";

interface Popover {
  backdrop: HTMLDivElement;
  container: HTMLDivElement;
  view: EmbeddedViewRef<unknown>;
}

@Injectable({
  providedIn: "root",
})
export class PopoverService {
  private readonly rendererFactory = inject(RendererFactory2);
  private readonly applicationRef = inject(ApplicationRef);
  private readonly injector = inject(Injector);

  private readonly renderer: Renderer2;

  private readonly opened = signal<EmbeddedViewRef<unknown>[]>([]);

  private readonly popoverContainer: HTMLDivElement;

  private readonly popovers = new Map<TemplateRef<unknown>, Popover>();

  constructor() {
    this.renderer = this.rendererFactory.createRenderer(null, null);

    const container = this.renderer.selectRootElement("body", true);

    this.popoverContainer = this.renderer.createElement("div");

    this.renderer.setAttribute(
      this.popoverContainer,
      "class",
      "absolute top-0 left-0 w-screen h-screen pointer-events-none dropdown dropdown-open overflow-hidden !block",
    );

    this.renderer.appendChild(container, this.popoverContainer);

    // this.popoverElement = this.renderer.createElement("div");
    // this.renderer.addClass(this.popoverElement, "dropdown");
    // this.renderer.addClass(this.popoverElement, "dropdown-open");
    // this.renderer.addClass(this.popoverElement, "h-screen");
    // this.renderer.addClass(this.popoverElement, "w-screen");
    // // this.renderer.addClass(this.popoverElement, "bg-gray-100");
    // this.renderer.addClass(this.popoverElement, "absolute");
    // this.renderer.addClass(this.popoverElement, "top-0");

    // this.popoverElement.addEventListener("mouseleave", () => {
    //   this.closeAll();
    // });

    // this.popoverElement.addEventListener("click", () => {
    //   this.closeAll();
    // });

    // this.renderer.appendChild(container, this.popoverElement);
  }

  readonly isPopupOpen = signal(false);

  readonly template = signal<TemplateRef<unknown> | undefined>(undefined);

  open(
    template: TemplateRef<unknown>,
    extra: {
      anchor: HTMLElement;
      context?: object;
      monitor?: number;
      injector?: Injector;
    },
  ): void {
    invoke("set_layer", {
      layer: "top",
      bar: extra.monitor ?? 0,
    }).then(() => {
      const backdrop = this.createBackdrop();

      console.log(extra.anchor);

      const container = this.createContainer();

      const view = template.createEmbeddedView(
        extra.context ?? {},
        extra.injector ? extra.injector : this.injector,
      );

      this.applicationRef.attachView(view);
      view.rootNodes.forEach((node) => {
        this.renderer.appendChild(container, node);
      });

      view.detectChanges();

      this.renderer.appendChild(this.popoverContainer, backdrop);
      this.renderer.appendChild(this.popoverContainer, container);

      console.log(container.clientWidth, extra.anchor.offsetTop);

      const [posX, posY] = this.getPosition(extra.anchor, container);

      this.renderer.setStyle(container, "top", `${posY}px`);
      this.renderer.setStyle(container, "left", `${posX}px`);

      backdrop.addEventListener("click", () => {
        this.renderer.removeChild(this.popoverContainer, container);
        this.renderer.removeChild(this.popoverContainer, backdrop);
        view.destroy();
        invoke("set_layer", {
          layer: "bottom",
          bar: extra.monitor ?? 0,
        });
      });

      container.addEventListener("mouseleave", () => {
        this.renderer.removeChild(this.popoverContainer, container);
        this.renderer.removeChild(this.popoverContainer, backdrop);
        view.destroy();

        invoke("set_layer", {
          layer: "bottom",
          bar: extra.monitor ?? 0,
        });
      });

      this.popovers.set(template, {
        backdrop,
        container,
        view,
      });
    });
  }

  private createBackdrop(): HTMLDivElement {
    const backdrop = this.renderer.createElement("div");
    this.renderer.setAttribute(
      backdrop,
      "class",
      "w-full h-full pointer-events-auto absolute top-0 left-0",
    );
    return backdrop;
  }

  private createContainer(): HTMLDivElement {
    const container = this.renderer.createElement("div");
    this.renderer.setAttribute(
      container,
      "class",
      "pointer-events-auto absolute dropdown-content !block",
    );

    return container;
  }

  private getPosition(
    anchor: HTMLElement,
    container: HTMLElement,
  ): [number, number] {
    const screenWidth = document.body.clientWidth;
    const screenHeight = document.body.clientHeight;

    const { clientWidth, clientHeight } = container;

    const x = anchor.offsetLeft;
    const y = anchor.offsetTop;

    let resX = anchor.offsetLeft;
    let resY = anchor.offsetTop + anchor.clientHeight;

    if (x + clientWidth > screenWidth) resX -= clientWidth - anchor.clientWidth;

    if (y + clientHeight > screenHeight)
      resY -= clientHeight + anchor.clientHeight;

    console.log({
      resX,
      resY,
      a: anchor.offsetTop,
      screenWidth,
      screenHeight,
      clientWidth,
      clientHeight,
    });

    return [resX, resY];
  }

  closeAll(): void {
    this.opened().forEach((popover) => {
      popover.destroy();
    });
    this.opened.set([]);
  }
}
