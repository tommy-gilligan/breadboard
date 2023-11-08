# Touchscreen

A touchscreen is just a screen that you can touch.  Central to this crate is
the Touchscreen trait:

```rust
pub trait Touchscreen:
    embedded_graphics_core::prelude::DrawTarget +
    embedded_graphics_core::prelude::OriginDimensions {
        fn get_touch_event(&mut self) -> Option<TouchEvent>;
    }
```

It is overly simple right now, it is more of a tentative starting point.  There
are 2 implementations, which can be selected by feature flags:

- `red-screen`: This is a screen that I had lying around which combines XPT2046 (touch) with ILI9488 (screen)
- `web-screen`: A screen that marries WebSimulatorDisplay with mouse event handlers bound to a container element.

## Todo

- Support touchscreens that can signal touch with interrupt
- Add an associated error type
- Add an SDL `touchscreen`
