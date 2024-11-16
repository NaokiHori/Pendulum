export function getHTMLElement(id: string): HTMLElement {
  const element: HTMLElement | null = document.getElementById(id);
  if (null === element) {
    throw new Error(`failed to find an element: ${id}`);
  }
  return element;
}

export function getSVGElement(id: string): SVGElement {
  const element: Element | null = document.getElementById(id);
  if (null === element) {
    throw new Error(`failed to find an element: ${id}`);
  }
  return element as SVGElement;
}

export function adjustSVGSize(container: HTMLElement, svg: SVGElement) {
  const rect: DOMRect = container.getBoundingClientRect();
  const width: number = rect.width;
  const height: number = rect.height;
  const size: number = height < width ? height : width;
  const sizeInPixel = `${size.toString()}px`;
  svg.style.width = sizeInPixel;
  svg.style.height = sizeInPixel;
}
