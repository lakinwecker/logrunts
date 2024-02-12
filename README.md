# logrunts

May not be useful for anyone else, but takes structured logging of the form of:
```json
{"environment":"development","level":"info","msg":"Starting 4 workers","name":"some-worker","time":"2024-02-12T09:49:12-07:00","trace-id":"00000000000000000000000000000000","version":"1.0"}
```
and turns it into:
```bash
INFO  worker/development@1.0 > Starting 4 workers
```

And if a trace is provided, it takes:
```json
{"environment":"development","level":"info","msg":"Starting 4 workers","name":"some-worker","time":"2024-02-12T09:49:12-07:00","trace-id":"00000000000000000001234567890000","version":"1.0"}
```
and turns it into:
```bash
INFO  analysis-worker/development@1.0 > 00000000000000000001234567890000 > Starting 4 workers
```

![Example output](./imgs/example.png?raw=true)

Intended to be used like:
```bash
go run cmds/worker.go | RUST_LOG=trace logrunts
```
