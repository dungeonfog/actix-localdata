# actix-localdata
`actix_web::web::Data`, but with `Rc` instead of `Arc`.

This is useful when you have something like a config struct, which doesn't take up
a whole lot of memory, but is also not small enough to copy all the time.

`LocalData` will allow you to have a local copy on each thread, which does not need
to be copied into handlers.

This is a fairly niche utility crate, so I'm not publishing it on crates for now.
