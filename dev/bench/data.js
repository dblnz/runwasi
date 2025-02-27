window.BENCHMARK_DATA = {
  "lastUpdate": 1740615430989,
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