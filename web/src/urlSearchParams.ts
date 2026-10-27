const MIN_NITEMS = 2;
const MAX_NITEMS = 10;

function getRandomNitems(): number {
  return Math.floor((MAX_NITEMS - 1) * Math.random() + MIN_NITEMS);
}

export function getNitems(): number {
  // check if URL param is given
  const urlParams = new URLSearchParams(window.location.search);
  if (urlParams.has("nitems")) {
    // if given, use after sanitised
    let nitems: number | null = Number(urlParams.get("nitems"));
    if (nitems) {
      nitems = Math.round(nitems);
      nitems = MIN_NITEMS < nitems ? nitems : MIN_NITEMS;
      return nitems;
    } else {
      // decide randomly if the input is invalid
      return getRandomNitems();
    }
  } else {
    // decide randomly
    return getRandomNitems();
  }
}
