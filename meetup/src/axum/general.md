# General

There are plenty of web application frameworks out there.
Other big examples are [warp](https://github.com/seanmonstar/warp), [actix-web](https://actix.rs/), or [rocket](https://rocket.rs/).

For our workshop, we will use [axum](https://github.com/tokio-rs/axum), the web framework created by the folks from [tokio](https://github.com/tokio-rs), as we have the most knowledge about with it (If you get stuck and e.g. need help).

But you can translate features, that axum offers also to
the other frameworks in the one or the other way. In the end the only thing you need is a framework that is capable of sending html
over the wire and process incoming http requests, which all of the above frameworks are capable of.
