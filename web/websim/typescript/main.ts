import wasm_wrapper_start, { WasmWrapper } from "./wasm_wrapper.js";

let wasm_wrapper_obj: WasmWrapper;

function set_pend_nitems(): number {
  // check if URL param is given
  const url_params = new URLSearchParams(window.location.search);
  let nitems: number = 2;
  if (url_params.has(`nitems`)) {
    // if given, use after sanitised
    let tmp: number | null = Number(url_params.get(`nitems`));
    if (tmp) {
      tmp = tmp < 8 ? tmp : 8;
      tmp = 1 < tmp ? tmp : 1;
      tmp = Math.floor(tmp);
      nitems = tmp;
    }
  } else {
    // decide randomly
    nitems = Math.floor(7. * Math.random() + 2.);
  }
  return nitems;
}

function update_and_draw(): void {
  // integrate in time to get new information
  wasm_wrapper_obj.integrate();
  // draw obects
  wasm_wrapper_obj.draw();
  // set myself as the callback
  requestAnimationFrame(update_and_draw);
}

window.addEventListener(`load`, () => {
  wasm_wrapper_start().then(() => {
    // all things which should be done before iterating
    const nitems: number = set_pend_nitems();
    // initialise simulator and drawer
    wasm_wrapper_obj = WasmWrapper.new(nitems);
    // register event handler for window size change
    window.addEventListener(`resize`, () => {
      wasm_wrapper_obj.update_canvas_size();
    });
    // trigger first animation flow
    update_and_draw();
  });
});

