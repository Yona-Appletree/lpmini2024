export interface ChangingImage {
  currentImage(): {
    image: CanvasImageSource;
    width: number;
    height: number;
  };
  addChangeListener: (listener: ImageChangedEvent) => () => void;
}

export type ImageChangedEvent = (image: CanvasImageSource) => void;
