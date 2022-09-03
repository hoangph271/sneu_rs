export function isWindowControlsOverlayVisible() {
  return !!navigator.windowControlsOverlay?.visible
}

export function setSrcObject(audioEl, blob) {
  audioEl.src = URL.createObjectURL(blob)
}
