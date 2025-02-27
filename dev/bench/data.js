window.BENCHMARK_DATA = {
  "lastUpdate": 1740615526702,
  "repoUrl": "https://github.com/dblnz/runwasi",
  "entries": {
    "Criterion.rs Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "828bad33924cda1f534fc7fb7be03c3555d44caf",
          "message": "Merge pull request #872 from containerd/dependabot/github_actions/sigstore/cosign-installer-3.8.1\n\nchore(deps): bump sigstore/cosign-installer from 3.8.0 to 3.8.1",
          "timestamp": "2025-02-25T23:48:04Z",
          "url": "https://github.com/dblnz/runwasi/commit/828bad33924cda1f534fc7fb7be03c3555d44caf"
        },
        "date": 1740615412493,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "wamr/memory-usage",
            "value": 15016,
            "unit": "kB",
            "extra": "shim: 12288 kB\nzygote: 2728 kB"
          },
          {
            "name": "wasmedge/memory-usage",
            "value": 66116,
            "unit": "kB",
            "extra": "shim: 53760 kB\nzygote: 12356 kB"
          },
          {
            "name": "wasmer/memory-usage",
            "value": 17792,
            "unit": "kB",
            "extra": "shim: 14464 kB\nzygote: 3328 kB"
          },
          {
            "name": "wasmtime/memory-usage",
            "value": 19264,
            "unit": "kB",
            "extra": "shim: 16000 kB\nzygote: 3264 kB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "828bad33924cda1f534fc7fb7be03c3555d44caf",
          "message": "Merge pull request #872 from containerd/dependabot/github_actions/sigstore/cosign-installer-3.8.1\n\nchore(deps): bump sigstore/cosign-installer from 3.8.0 to 3.8.1",
          "timestamp": "2025-02-25T23:48:04Z",
          "url": "https://github.com/dblnz/runwasi/commit/828bad33924cda1f534fc7fb7be03c3555d44caf"
        },
        "date": 1740615525661,
        "tool": "cargo",
        "benches": [
          {
            "name": "end-to-end/wasmtime/wasi-demo-app:latest",
            "value": 135730759,
            "range": "± 2619544",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmtime/wasi-demo-oci:latest",
            "value": 80426079,
            "range": "± 1044370",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmedge/wasi-demo-app:latest",
            "value": 109069053,
            "range": "± 1637439",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmedge/wasi-demo-oci:latest",
            "value": 119122729,
            "range": "± 2643635",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmer/wasi-demo-app:latest",
            "value": 130807073,
            "range": "± 4896575",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmer/wasi-demo-oci:latest",
            "value": 132539248,
            "range": "± 3794743",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wamr/wasi-demo-app:latest",
            "value": 79617390,
            "range": "± 2034671",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wamr/wasi-demo-oci:latest",
            "value": 83995324,
            "range": "± 977512",
            "unit": "ns/iter"
          }
        ]
      }
    ],
    "HTTP Throughput": [
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "828bad33924cda1f534fc7fb7be03c3555d44caf",
          "message": "Merge pull request #872 from containerd/dependabot/github_actions/sigstore/cosign-installer-3.8.1\n\nchore(deps): bump sigstore/cosign-installer from 3.8.0 to 3.8.1",
          "timestamp": "2025-02-25T23:48:04Z",
          "url": "https://github.com/dblnz/runwasi/commit/828bad33924cda1f534fc7fb7be03c3555d44caf"
        },
        "date": 1740615428034,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "HTTP RPS",
            "value": 20639.8611,
            "unit": "req/s"
          }
        ]
      }
    ],
    "HTTP Latency": [
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "828bad33924cda1f534fc7fb7be03c3555d44caf",
          "message": "Merge pull request #872 from containerd/dependabot/github_actions/sigstore/cosign-installer-3.8.1\n\nchore(deps): bump sigstore/cosign-installer from 3.8.0 to 3.8.1",
          "timestamp": "2025-02-25T23:48:04Z",
          "url": "https://github.com/dblnz/runwasi/commit/828bad33924cda1f534fc7fb7be03c3555d44caf"
        },
        "date": 1740615430345,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "HTTP p95 Latency",
            "value": 4.3,
            "unit": "ms"
          }
        ]
      }
    ]
  }
}