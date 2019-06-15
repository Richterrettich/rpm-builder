## RPM-BUILDER

Build rpms without SPEC files. This is written in pure Rust, so it should be easy to do static linking.

### Example

```bash
rpm-builder \ 
  --exec-file "path/to/binary:/usr/bin/awesome-bin" \
  --config-file "path/to/config-file:/etc/awesome/config.json" \
  --doc-file "path/to/doc-file:/usr/share/man/awesome/manpage.1.gz" \
  --changelog "me:was awesome, eh?:2018-01-02" \
  --changelog "you:yeah:2018-01-02" \
  --requires "wget >= 1.0.0" \
  --obsoletes "rpmbuild" \
  awesome
# creates a file called awesome.rpm in version 1.0.0, release 1, license is MIT.
```

