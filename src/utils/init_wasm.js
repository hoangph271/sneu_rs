export function isWindowControlsOverlayVisible() {
  return !!navigator.windowControlsOverlay?.visible
}

export function setSrcObject(mediaEl, blob) {
  mediaEl.src = URL.createObjectURL(blob)
}
