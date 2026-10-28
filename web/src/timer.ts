export class Timer {
  private counter: number;
  private increment: number;
  private startAt: number;
  private onTimerReset: () => void;

  public constructor(increment: number, onTimerReset: () => void) {
    this.counter = 0;
    this.increment = increment;
    this.startAt = 0;
    this.onTimerReset = onTimerReset;
  }

  public start(): void {
    this.startAt = performance.now();
  }

  public update(): void {
    this.counter += 1;
    if (this.increment < performance.now() - this.startAt) {
      console.log(`${this.counter.toString()} animation loops per ${this.increment.toString()} ms`);
      this.onTimerReset();
      this.counter = 0;
      this.startAt = performance.now();
    }
  }
}
