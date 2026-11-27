# Honkai: Star Rail fork

This checkout is a fork of `tuanminh26596/Genshin-helper` with an HSR-only
target boundary. The port deliberately keeps the passive, image-based product
shape, but does not reuse Genshin's Teyvat calibration or `ys_obc` API contract.

## Verified HSR provider contract

- Interactive map: `https://act.hoyolab.com/sr/app/interactive-map/`
- API base: `https://sg-act-public-api-static.hoyolab.com/common/srmap/sr_map`
- API app sign: `sr_map`
- Required app version: `de16a09fca4e0ab89acf69fe0c12514f`
- Map IDs currently advertised by the official map bundle: `38..54`
- A point list request must include `map_id`, `app_sn`, `lang`, and
  `app_version`.

The URL builder lives in `gh-net::hsr_map`. It is isolated from the legacy
Genshin sync client so a future multi-game host can select a provider by game
profile rather than by changing global constants.

## Safety boundary

The HSR minimap tracker is currently fail-closed. The existing CV tracker is
calibrated for Teyvat; running it against Star Rail would produce inaccurate
coordinates. Until HSR-specific minimap anchors, map projection, and a ground-
truth calibration set are added, the app must not publish a pose or auto-sync
one.

## Next implementation slice

1. Capture HSR minimap screenshots from the supported window without memory
   access or input injection.
2. Build an HSR minimap detector and projection calibration against map IDs
   `38..54`.
3. Add fixture tests for stationary, pan, teleport, loading, and multi-floor
   transitions.
4. Wire the new HSR provider into the browser extension only after the native
   adapter passes deterministic URL/JSON and coordinate tests.
