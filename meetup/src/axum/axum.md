# Axum

[Axum](https://github.com/tokio-rs/axum), the web framework created by the folks from [tokio](https://github.com/tokio-rs).

What does axum feature, or set them apart from other frameworks:

- Macro free.
- Parse requests using extractors. E.g. to extract JSON in a handler.
- Has no own middleware - relies on integrating tower and tower-http and thus making it quite modular.

So what will we show you, that you will need later on:

- How to write a handler in axum and return data as http response
- How to extract payload from an incoming request (Extractors)
- How to add state to you application
