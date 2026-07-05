- [x] (F) More file types ::
    * `XffValue` should be ready to support more file types already, it would make sense to keep them unified in one place.
    * Keep in mind: https://martin-ueding.de/posts/json-vs-yaml-vs-toml/ and https://www.anbowell.com/blog/an-in-depth-comparison-of-json-yaml-and-toml/
    - [x] TOML :: Somewhere between `JSON` and `YAML`. No brainer.
        * Spec: https://toml.io/en/v1.1.0
        * Very useful (includes full example): https://www.anbowell.com/blog/the-developers-guide-to-toml/
        - [ ] No emoji support :: Because Thoth does not support it. Add to README
    - [c] YAML :: DIFFICULT, probably not worth it
- [x] (F) Feature rework :: lock each file type behind a feature
    * Except `JSON` as standard, maybe provide a `no_json` feature (I know features should only be additive but fuck it).
