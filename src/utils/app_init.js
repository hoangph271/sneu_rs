export function name() {
  return 'Rust';
}

export class MyClass {
  constructor() {
    this._number = 42;
  }

  get number() {
    return this._number;
  }

  set number(n) {
    return this._number = n;
  }

  render() {
    console.info(window.WindowControlsOverlay)
    return `My number is: ${this.number}, ${1}`;
  }
}
