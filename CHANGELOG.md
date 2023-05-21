# Changelog

## Version 0.3.0 (2023-05-21)

### New Features

* Add non-wasm support
    * This isn't heavily tested, but seems to be working

### Breaking Changes

* `category` now uses an enum `Category` instead of `str`

### Internal Changes

* Use `IoTaskPool` instead of `AsyncComputeTaskPool`

## Version 0.2.0 (2023-05-07)

* Add support for all remaining device types and effect types
* `BGRColor` now multiplies color channels with the alpha channel value
    * Note that this also affects `KeyColor`
* Implement common traits on all public structs, such as `Hash`, `Eq`, etc
* Implement Serialize and Deserialize for all API structs
    * This should facilitate serializing the data to files, etc
* `CreateEffectResponse` is now private to the crate; it was never intended to be exposed
* Create new SystemSet `HttpRequestSet` which is run after `CoreSet::PostUpdate` and before `CoreSet::PostUpdateFlush`
    * `ChromaPlugin` and `HttpRequestPlugin` are now run at more appropriate times in this set
    * Full list of entries in this set are as follows:
        * `BeforeExecuteRequests`,
            * Create and Apply effects requests are prepared here
        * `ExecuteRequests`
            * Http requests are dispatched here
        * `AfterExecuteRequests`,
        * `GatherResponses`,
            * Responses to http requests are gathered here
        * `AfterGatherResponses`,
            * Responses to Create and Apply effect reponses are gathered here
* `device_supported` now uses an enum `SupportedDevice` instead of `str`s


## Version 0.1.1 (2023-04-30)

* Add initial support for keyboard
* Disable default Bevy features in Cargo.toml
* Rearrange and clean up imports and exports

## Version 0.1.0 (2023-04-30)

* Initial release with mouse support only
