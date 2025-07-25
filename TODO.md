# TODO

Core functionality is there.

We have a 'technically' a regression in the sense that, the default routing isn't to open the
default prog + launcher, but the remainder logic (of primary vs. clip and writing/retrieving)
is there.

- Technical regression - default behaviour is no longer to 'open the launcher'
  This needs to be resolved asap.

- 'unoccupied seat' when calling certain types of logic.

- Duplicated logic in cache.rs and selector.rs

- Need to ask original author if the 'extra' number/timestamp entries are actually supposed
  to be getting logged to the clipboard (IndexMap) or if that was a bug in the original one,
  and it's just happened to carry over as I ported things.

## TODO/Improvements

- Message passing for async context cross-over
  - This is a big one, as it will allow us to have a more robust async context
  - It will also allow us to have a more robust error handling mechanism

- Binary/VarBinary support types for image copying.

- Enviornment overrides for various config params
