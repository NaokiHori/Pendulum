export function getNitems(): number {
  // check if URL param is given
  const urlParams = new URLSearchParams(window.location.search);
  let nitems = 2;
  const maxNitems = 8;
  if (urlParams.has("nitems")) {
    // if given, use after sanitised
    let tmp: number | null = Number(urlParams.get("nitems"));
    if (tmp) {
      tmp = tmp < maxNitems ? tmp : maxNitems;
      tmp = 1 < tmp ? tmp : 1;
      tmp = Math.floor(tmp);
      nitems = tmp;
    }
  } else {
    // decide randomly
    nitems = Math.floor((maxNitems - 1) * Math.random() + 2);
  }
  return nitems;
}
