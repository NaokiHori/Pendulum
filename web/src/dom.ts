export function getElementByIdOrThrow<T extends Element>(
  type: new (...args: unknown[]) => T,
  id: string,
): T {
  const element = document.querySelector(`#${id}`);
  if (null === element) {
    throw new Error();
  }
  if (!(element instanceof type)) {
    throw new TypeError();
  }
  return element;
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
