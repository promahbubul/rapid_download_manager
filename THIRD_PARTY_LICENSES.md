# Third-Party Open Source Software Notices & Licenses

**Product:** Rapid Download Manager  
**Publisher:** Promahbubul  
**Version:** 1.0.0  

---

## 1. Licensing & Legal Compliance Overview

Rapid Download Manager relies on a curated set of battle-tested open-source crates from the Rust ecosystem. A comprehensive legal audit of all dependencies in `Cargo.lock` has been conducted to verify distribution compatibility with both standalone distribution and the Microsoft Store:

* **100% Permissive Stack:** All direct and transitive dependencies utilize standard permissive open-source licenses (`MIT`, `Apache-2.0`, `BSD-3-Clause`, `ISC`, `Zlib`, and `Unicode-3.0`).
* **Zero Copyleft Obligations:** No dependencies utilize GPL, AGPL, or restrictive copyleft licenses. There are zero requirements to disclose proprietary source code or comply with viral licensing clauses.
* **Store & Commercial Ready:** Fully compliant with Microsoft Store Developer Agreement and international open-source copyright guidelines.

---

## 2. Direct Runtime Dependencies

The following table documents all direct dependencies utilized by Rapid Download Manager along with their respective versions and license classifications:

| Crate Name | Installed Version | License Classification | Primary Function |
| :--- | :---: | :--- | :--- |
| **`tokio`** | `1.53.1` | MIT | Multi-threaded asynchronous execution engine |
| **`reqwest`** | `0.12.28` | MIT OR Apache-2.0 | HTTP/2 and pure Rustls TLS client |
| **`eframe`** | `0.28.1` | MIT OR Apache-2.0 | Native desktop application shell and event loop |
| **`egui`** | `0.28.1` | MIT OR Apache-2.0 | High-performance immediate-mode GUI engine |
| **`serde`** | `1.0.229` | MIT OR Apache-2.0 | Data serialization framework |
| **`serde_json`** | `1.0.151` | MIT OR Apache-2.0 | JSON state and database parsing |
| **`chrono`** | `0.4.45` | MIT OR Apache-2.0 | Date, time, and scheduler synchronization |
| **`regex`** | `1.13.1` | MIT OR Apache-2.0 | High-performance regular expression engine |
| **`url`** | `2.5.8` | MIT OR Apache-2.0 | WhatWG compliant URL parsing and validation |
| **`clap`** | `4.6.7` | MIT OR Apache-2.0 | Command-line argument parser for CLI binary |
| **`indicatif`** | `0.17.11` | MIT | Multi-progress terminal visualizer |
| **`arboard`** | `3.6.1` | MIT OR Apache-2.0 | Native Windows clipboard integration |
| **`rfd`** | `0.14.1` | MIT | Native Windows file explorer picker dialogs |
| **`open`** | `5.4.4` | MIT | File Explorer and browser URL opener |
| **`anyhow`** | `1.0.104` | MIT OR Apache-2.0 | Idiomatic application error handling |
| **`thiserror`** | `2.0.20` | MIT OR Apache-2.0 | Structured domain error definitions |
| **`bytes`** | `1.12.1` | MIT | Memory-efficient byte chunk buffers |
| **`tokio-util`** | `0.7.19` | MIT | Async stream utilities and cancellation tokens |
| **`log`** | `0.4.34` | MIT OR Apache-2.0 | Structured logging facade |
| **`urlencoding`** | `2.1.3` | MIT | URL percentage encoding and decoding |

---

## 3. Standard Permissive License Texts

### The MIT License
```text
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### The Apache License, Version 2.0
```text
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
