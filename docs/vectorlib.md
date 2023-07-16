# VectorLib
This library I have created for handling the `2d` vectors and we are utilizing
it in our `Engine` for graphics and animations in Rust using `SDL2`.

## Modifications
- Before we have utilized the `vectorlib` library in the parent directory as
you can see it here `PATH = ../../vectorlib/`, and we fetched it to our
`engine` project using `Cargo.tomel` as:

```tomel
vectorlib = {path = "../vectorlib"}
```
- Now, we will use the uploaded ones, we download it similar to any other `Crate` from `Crate.io` hub.

```tomel
vectorlib = "0.1.2"
```


