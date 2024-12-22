import { checkUseLogo } from "./logo";
import logoSvg from "./logo.svg?raw";

export function initSVGObjects(
  svg: SVGElement,
  nitems: number,
): { lines: Element[]; circles: Element[] } {
  const svgSize: number = nitems + 0.5;
  const radius = 0.25;
  const lineWidth = 0.05;
  // decide view box
  svg.setAttribute(
    "viewBox",
    `-${svgSize.toString()} -${svgSize.toString()} ${(2 * svgSize).toString()} ${(2 * svgSize).toString()}`,
  );
  // initialize objects inside svg
  const namespaceURI = "http://www.w3.org/2000/svg";
  const lines = new Array<Element>();
  for (let i = 0; i < nitems; i++) {
    const line: Element = document.createElementNS(namespaceURI, "line");
    line.setAttribute("stroke", "#fff");
    line.setAttribute("stroke-width", lineWidth.toString());
    line.setAttribute("stroke-linecap", "round");
    svg.appendChild(line);
    lines.push(line);
  }
  const useLogo: boolean = checkUseLogo(nitems);
  const circles = new Array<Element>();
  if (useLogo) {
    for (let i = 0; i < nitems; i++) {
      const circle: Element = document.createElementNS(namespaceURI, "g");
      circle.innerHTML = logoSvg;
      svg.appendChild(circle);
      circles.push(circle);
    }
  } else {
    for (let i = 0; i < nitems; i++) {
      const circle: Element = document.createElementNS(namespaceURI, "circle");
      circle.setAttribute("r", radius.toString());
      const hue = Math.round((360 * i) / nitems);
      circle.setAttribute("fill", `hsl(${hue.toString()}deg 100% 80%)`);
      svg.appendChild(circle);
      circles.push(circle);
    }
  }
  return { lines, circles };
}

export function drawSVGObjects(
  nitems: number,
  positions: Float64Array,
  lines: Element[],
  circles: Element[],
) {
  for (const [index, line] of lines.entries()) {
    const x1: number = 0 == index ? 0 : positions[2 * index - 2];
    const y1: number = 0 == index ? 0 : positions[2 * index - 1];
    const x2: number = positions[2 * index + 0];
    const y2: number = positions[2 * index + 1];
    line.setAttribute("x1", x1.toString());
    line.setAttribute("y1", y1.toString());
    line.setAttribute("x2", x2.toString());
    line.setAttribute("y2", y2.toString());
  }
  const useLogo: boolean = checkUseLogo(nitems);
  for (const [index, circle] of circles.entries()) {
    const x: number = positions[2 * index];
    const y: number = positions[2 * index + 1];
    if (useLogo) {
      circle.setAttribute(
        "transform",
        `translate(${x.toString()}, ${y.toString()}) scale(0.005)`,
      );
    } else {
      circle.setAttribute("cx", x.toString());
      circle.setAttribute("cy", y.toString());
    }
  }
}
