type PendulumPointer = number & { readonly __brand: unique symbol };
type CartesianPositionsPointer = number & { readonly __brand: unique symbol };

type PendulumExports = {
  memory: WebAssembly.Memory;
  pendulum_new: (nitems: number) => PendulumPointer;
  pendulum_get_cartesian_positions: (pendulumPointer: PendulumPointer) => CartesianPositionsPointer;
  pendulum_integrate: (pendulumPointer: PendulumPointer) => number;
  pendulum_destroy: (pendulum: PendulumPointer) => void;
};

const EXPECTED_ARITY = {
  pendulum_new: 1,
  pendulum_get_cartesian_positions: 1,
  pendulum_integrate: 1,
  pendulum_destroy: 1,
} as const;

function isPendulumExports(exports: WebAssembly.Exports): exports is PendulumExports {
  return (
    exports.memory instanceof WebAssembly.Memory &&
    Object.entries(EXPECTED_ARITY).every(([name, arity]) => {
      const fn = exports[name];
      return typeof fn === "function" && fn.length === arity;
    })
  );
}

export class Pendulum implements Disposable {
  private exports: PendulumExports;
  private pendulumPointer: PendulumPointer | null;
  private nitems: number;

  public constructor(wasmInstance: WebAssembly.Instance, nitems: number) {
    const exports = wasmInstance.exports;
    if (!isPendulumExports(exports)) {
      throw new TypeError("WASM does not provide expected exports");
    }
    this.exports = exports;
    this.pendulumPointer = exports.pendulum_new(nitems);
    this.nitems = nitems;
  }

  private getPendulumPointerOrThrow(): PendulumPointer {
    const pendulumPointer = this.pendulumPointer;
    if (pendulumPointer === null) {
      throw new Error("invalid pendulum pointer");
    }
    return pendulumPointer;
  }

  public integrate(): number {
    const pendulumPointer = this.getPendulumPointerOrThrow();
    return this.exports.pendulum_integrate(pendulumPointer);
  }

  public getCartesianPositions(): Float64Array {
    const pendulumPointer = this.getPendulumPointerOrThrow();
    return new Float64Array(
      this.exports.memory.buffer,
      this.exports.pendulum_get_cartesian_positions(pendulumPointer),
      this.nitems * 2,
    );
  }

  public [Symbol.dispose](): void {
    if (this.pendulumPointer === null) {
      return;
    }
    const pendulumPointer = this.pendulumPointer;
    this.pendulumPointer = null;
    this.exports.pendulum_destroy(pendulumPointer);
  }
}
