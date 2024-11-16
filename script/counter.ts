export class Counter {
  private _counter: number;
  private _counterMax: number;

  public constructor(counterMax: number) {
    this._counter = 0;
    this._counterMax = counterMax;
  }

  public get(): number {
    return this._counter;
  }

  public update() {
    this._counter += 1;
    if (this._counterMax < this._counter) {
      this._counter = 0;
    }
  }
}
