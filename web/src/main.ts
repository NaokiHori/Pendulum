import { getElementByIdOrThrow, adjustSVGSize } from "./dom";
import { getNitems } from "./urlSearchParams";
import { initSVGObjects, drawSVGObjects } from "./svgObjects";
import { Timer } from "./timer";
import { Pendulum } from "./pendulum";
import wasmUrl from "./pendulum.wasm?url";

async function getWasmInstance(): Promise<WebAssembly.Instance> {
  const { instance } = await WebAssembly.instantiateStreaming(fetch(wasmUrl));
  return instance;
}

async function main() {
  const wasmInstance = await getWasmInstance();
  // number of masses, given by user or decide randomly
  const nitems: number = getNitems();
  // fetch DOMs
  const container = getElementByIdOrThrow(HTMLElement, "svg-container");
  const svgElement = getElementByIdOrThrow(SVGElement, "svg-canvas");
  // create lines and circles inside the given SVG image
  const { lines, circles }: { lines: Element[]; circles: Element[] } = initSVGObjects(
    svgElement,
    nitems,
  );
  // initialize simulator
  const pendulum = new Pendulum(wasmInstance, nitems);
  // make the canvas (container / image) full-screen
  adjustSVGSize(container, svgElement);
  // register synchronization as an event as well
  window.addEventListener("resize", () => {
    adjustSVGSize(container, svgElement);
  });
  const timer = new Timer(1000, () => {
    /* nothing to do */
  });
  // rendering loop
  function updateAndDraw() {
    // integrate in time to get new information
    const drawFreq = 5e-2;
    for (let time = 0; ;) {
      const dt: number = pendulum.integrate();
      time += dt;
      if (drawFreq < time) {
        break;
      }
    }
    // create a view to the shared memory
    // NOTE: this should be called every time as the pointer seems to be changed
    const positions = pendulum.getCartesianPositions();
    drawSVGObjects(nitems, positions, lines, circles);
    timer.update();
    requestAnimationFrame(() => {
      updateAndDraw();
    });
  }
  // trigger first rendering
  timer.start();
  updateAndDraw();
}

window.addEventListener("load", () => {
  main().catch((error: unknown) => {
    if (error instanceof Error) {
      console.error(error);
    } else {
      throw error;
    }
  });
});
