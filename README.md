# rpm-builder

Build rpms without SPEC files. This is written in pure Rust, so it should be
easy to do static linking.

## Example

```bash
rpm-builder \
  --exec-file "path/to/binary:/usr/bin/awesome-bin" \
  --config-file "path/to/config-file:/etc/awesome/config.json" \
  --doc-file "path/to/doc-file:/usr/share/man/awesome/manpage.1.gz" \
  --compression gzip \
  --changelog "me:was awesome, eh?:2018-01-02" \
  --changelog "you:yeah:2018-01-02" \
  --requires "wget >= 1.0.0" \
  --obsoletes "rpmbuild" \
  awesome
# creates a file called awesome.rpm in version 1.0.0, release 1, license is MIT.
```

## Additional Flags

| Flag          | Description                                                                                                   |
| ---           | ---                                                                                                           |
| `arch`        | Specify the target architecture                                                                               |
| `changelog`   | Add a changelog entry to the rpm. The entry has the form `<author>:<content>:<yyyy-mm-dd>` (time is in utc)   |
| `compression` | Specify the compression algorithm. Currently only gzip is supported                                           |
| `config-file` | Add a config-file to the rpm                                                                                  |
| `conflicts`   | Indicates that the rpm conflicts with another package. Use the format `<name> [> | >= | = | <= | < version]`  |
| `desc`        | Give a description of the package                                                                             |
| `dir`         | Add a directory and all its files to the rpm                                                                  |
| `doc-file`    | Add a documentation-file to the rpm                                                                           |
| `exec-file`   | Add a executable-file to the rpm                                                                              |
| `file`        | Add a regular file to the rpm                                                                                 |
| `license`     | Specify a license                                                                                             |
| `name`        | Specify the name of your package                                                                              |
| `obsoletes`   | Indicates that the rpm obsoletes another package. Use the format `<name> [> | >= | = | <= | < version]`       |
| `out`         | Specify an out file                                                                                           |
| `provides`    | Indicates that the rpm provides another package. Use the format `<name> [> | >= | = | <= | < version]`        |
| `release`     | Specify release number of the package                                                                         |
| `requires`    | Indicates that the rpm requires another package. Use the format `<name> [> | >= | = | <= | < version]`        |
| `version`     | Specify a version                                                                                             |
