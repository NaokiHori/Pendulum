import initWasm, { Wrapper, InitOutput, Energy } from "../pkg";
import { getHTMLElement, getSVGElement, adjustSVGSize } from "./dom";
import { getNitems } from "./urlSearchParams";
import { initSVGObjects, drawSVGObjects } from "./svgObjects";
import { Timer } from "./timer";

async function main() {
  const wasm: InitOutput = await initWasm();
  // number of masses, given by user or decide randomly
  const nitems: number = getNitems();
  // fetch DOMs
  const container: HTMLElement = getHTMLElement("svg-container");
  const svgElement: SVGElement = getSVGElement("svg-canvas");
  // create lines and circles inside the given SVG image
  const { lines, circles }: { lines: Element[]; circles: Element[] } =
    initSVGObjects(svgElement, nitems);
  // initialize simulator
  const wrapper = new Wrapper(nitems);
  // make the canvas (container / image) full-screen
  adjustSVGSize(container, svgElement);
  // register synchronization as an event as well
  window.addEventListener("resize", () => {
    adjustSVGSize(container, svgElement);
  });
  // compute and print energies periodically
  // this is registered to the timer (profiler)
  const handleTimerReset = (): void => {
    const energies: Energy = wrapper.check_energies();
    const kinetic: number = energies.kinetic;
    const potential: number = energies.potential;
    const total: number = kinetic + potential;
    console.log(
      `kinetic: ${kinetic.toString()} potential: ${potential.toString()} total: ${total.toString()}`,
    );
  };
  const timer = new Timer(1000, handleTimerReset);
  // animation kernel
  function updateAndDraw() {
    // integrate in time to get new information
    wrapper.integrate();
    // create a view to the shared memory
    // NOTE: this should be called every time as the pointer seems to be changed
    const positions = new Float64Array(
      wasm.memory.buffer,
      wrapper.get_positions(),
      nitems * 2,
    );
    drawSVGObjects(nitems, positions, lines, circles);
    timer.update();
    // set myself as the callback
    requestAnimationFrame(updateAndDraw);
  }
  // trigger first animation flow
  updateAndDraw();
}

window.addEventListener("load", () => {
  main().catch((error: unknown) => {
    throw error;
  });
});
