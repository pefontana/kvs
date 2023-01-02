# kvs
A command-line and library key-value store

The `kvs` executable supports the following command line arguments:

- `kvs set <KEY> <VALUE>`

  Set the value of a string key to a string.
  Print an error and return a non-zero exit code on failure.

- `kvs get <KEY>`

  Get the string value of a given string key.
  Print an error and return a non-zero exit code on failure.

- `kvs rm <KEY>`

  Remove a given key.
  Print an error and return a non-zero exit code on failure.

- `kvs -V`

  Print the version

The `kvs` library contains a type, `KvStore`, that supports the following
methods:

- `KvStore::set(&mut self, key: String, value: String) -> Result<()>`

  Set the value of a string key to a string.
  Return an error if the value is not written successfully.

- `KvStore::get(&mut self, key: String) -> Result<Option<String>>`

  Get the string value of a string key.
  If the key does not exist, return `None`.
  Return an error if the value is not read successfully.

- `KvStore::remove(&mut self, key: String) -> Result<()>`

  Remove a given key.
  Return an error if the key does not exist or is not removed successfully.

- `KvStore::open(path: impl Into<PathBuf>) -> Result<KvStore>`

  Open the KvStore at a given path.
  Return the KvStore.

When setting a key to a value, `kvs` writes the `set` command to disk in a
sequential log, then stores the log pointer (file offset) of that command in the
in-memory index from key to pointer. When removing a key, similarly, `kvs`
writes the `rm` command in the log, then removes the key from the in-memory
index.  When retrieving a value for a key with the `get` command, it searches
the index, and if found then loads from the log the command at the corresponding
log pointer, evaluates the command and returns the result.

On startup, the commands in the log are traversed from oldest to newest, and the
in-memory index rebuilt.

When the size of the uncompacted log entries reach a given threshold, `kvs`
compacts it into a new log, removing redundent entries to reclaim disk space.

