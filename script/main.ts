import wbgInit, { Pendulum, InitOutput, Energy } from "../pkg";
import { getHTMLElement, getSVGElement, adjustSVGSize } from "./dom";
import { getNitems } from "./urlSearchParams";
import { initSVGObjects, drawSVGObjects } from "./svgObjects";
import { Timer } from "./timer";

async function main() {
  const wbgModule: InitOutput = await wbgInit();
  // number of masses, given by user or decide randomly
  const nitems: number = getNitems();
  // fetch DOMs
  const container: HTMLElement = getHTMLElement("svg-container");
  const svgElement: SVGElement = getSVGElement("svg-canvas");
  // create lines and circles inside the given SVG image
  const { lines, circles }: { lines: Element[]; circles: Element[] } =
    initSVGObjects(svgElement, nitems);
  // initialize simulator
  const pendulum = new Pendulum(nitems);
  // make the canvas (container / image) full-screen
  adjustSVGSize(container, svgElement);
  // register synchronization as an event as well
  window.addEventListener("resize", () => {
    adjustSVGSize(container, svgElement);
  });
  // compute and print energies periodically
  // this is registered to the timer (profiler)
  const handleTimerReset = (): void => {
    const energies: Energy = pendulum.check_energies();
    const kinetic: number = energies.kinetic;
    const potential: number = energies.potential;
    const total: number = kinetic + potential;
    console.log(`  kinetic: ${kinetic.toString()}`);
    console.log(`  potential: ${potential.toString()}`);
    console.log(`  total: ${total.toString()}`);
  };
  const timer = new Timer(1000, handleTimerReset);
  // rendering loop
  function updateAndDraw() {
    // integrate in time to get new information
    const drawFreq = 5e-2;
    for (let time = 0; ; ) {
      const dt: number = pendulum.integrate();
      time += dt;
      if (drawFreq < time) {
        break;
      }
    }
    // create a view to the shared memory
    // NOTE: this should be called every time as the pointer seems to be changed
    const positions = new Float64Array(
      wbgModule.memory.buffer,
      pendulum.get_cartesian_positions(),
      nitems * 2,
    );
    drawSVGObjects(nitems, positions, lines, circles);
    timer.update();
    requestAnimationFrame(updateAndDraw);
  }
  // trigger first rendering
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
