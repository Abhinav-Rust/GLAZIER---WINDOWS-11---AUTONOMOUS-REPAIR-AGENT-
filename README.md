# GLAZIER 🪟

**A Deterministic, Symbolic Windows 11 Repair Agent Grounded in Classical Indian Logic**

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![Platform: Windows 11](https://img.shields.io/badge/Platform-Windows_11_Pro-blue.svg)

---

## 🛑 The "One-Sentence Test" & Why No LLMs

**GLAZIER is a narrow symbolic AI. It knows two things: Windows 11 and English. Nothing else.**

Most modern AI projects are wrappers around Large Language Models (LLMs). For system repair, relying on an LLM is like asking a scholar who has read every book ever written to fix a leaking pipe — the knowledge is buried, probabilistic, and prone to hallucination.

GLAZIER rejects this trend. Its intelligence comes entirely from **logical structure**, not statistical weights.
* **Hallucination is architecturally impossible.** GLAZIER cannot invent a fix it has not been explicitly taught.
* **Every decision is 100% auditable.** Every conclusion traces to a named rule, a witnessed case, or a direct observation.
* **Knowledge grows by addition.** Adding a new Windows error code means writing a new deterministic rule block, not retraining a billion-parameter model.

---

## 🏛️ Philosophical Foundation

GLAZIER's cognitive architecture is grounded entirely in classical Indian knowledge systems, which predate Western equivalents by centuries and excel in philosophical rigor and practical precision.

| Layer | Foundation | Period | Application in GLAZIER |
|-------|-----------|--------|------------------------|
| **Epistemology** | Nyāya Pramāṇas | ~2nd century BCE | Direct Observation (Pratyakṣa), Inference (Anumāna), Analogy (Upamāna), Authority (Śabda) |
| **Ontology** | Vaiśeṣika Padārthas | ~2nd century BCE | 7 Strict Categories (Dravya, Guṇa, Karma, Sāmānya, Viśeṣa, Samavāya, Abhāva) |
| **Reasoning** | Nyāya Anumāna + Vyāpti | ~2nd century BCE | Five-member syllogism; all inferences grounded in witnessed cases (Udāharaṇa) |
| **Conflict Resolution**| Kauṭilya Caturupāya | ~4th century BCE | Ordered fallback escalation (Sāma → Dāna → Bheda → Daṇḍa) |
| **Conflict Classification**| Sāṃkhya Guṇas | ~4th century BCE | System state tracking (Tamas: Inertia, Rajas: Conflict, Sattva: Harmony) |
| **Grammar** | Pāṇini Aṣṭādhyāyī | ~4th century BCE | Unambiguous, highly compressed domain-specific language (WRL) |
| **Fallacy Guard** | Nyāya Hetvābhāsa | ~2nd century BCE | Pre-execution blocking of circular, contradictory, or unproven logic |

---

## ⚙️ The Three Components

This repository contains three tightly integrated components:

### 1. GLAZIER (The Runtime Agent)
Written in **Rust**, the agent is the execution and reasoning engine. It utilizes a Glushkov NFA and Markov-style Trie transducer for strict Natural Language Understanding (NLU). It features a **bifurcated execution layer**, allowing the epistemological core to run safely in a Linux sandbox (`SandboxMock`) while being cross-compilable for native Windows 11 API interaction (`Win32Native`). It uses **SurrealDB (RocksDB)** for persistent episodic memory.

### 2. WRL (Windows Repair Language)
A custom Domain-Specific Language (DSL) built using a `pest` PEG parser. Structured on Pāṇinian principles (Sūtra, Adhikāra, Anuvṛtti), WRL uses precise Sanskrit terminology for logical bounds and English for domain operations. WRL enforces strict adherence to the Vaiśeṣika ontology.

### 3. KB (The Knowledge Base)
A collection of `.wrl` files acting as the authoritative Śabda (testimony). The repository seeds an initial Windows 11 Audio Repair Knowledge Base, detailing dependency hierarchies, error codes (e.g., Code 43), and driver resolution paths.

---

## 🔍 The Trace Log Standard

GLAZIER is built for transparency. A human must be able to verify every step independently. When GLAZIER executes, it generates a strict epistemological trace:

```text
ANUMANA [primary_diagnosis]
  pramana_source:   [PRATYAKSHA]
  hetu:             error_code(43) observed at [2026-05-16T15:31:06Z]
  vyapti:           [SHABDA] post_update_driver_conflict
  witnessed:        [UPAMANA] [case_seed_001]
  hetvabhasa_check: PASSED
  nigamana:         rollback_driver(realtek_audio)

ACTION [rollback_driver]
  before_state:     Mock Before State
  action_taken:     rollback_driver
  after_state:      Mock After State
  guna_end:         sattva
  rollback_stored:  yes
```

---

## 🚀 Building & Running

Ensure you have Rust and Cargo installed.

```bash
# Clone the repository
git clone https://github.com/your-username/glazier.git
cd glazier

# Run the agent in the local sandbox
cargo run --bin glazier

# To test the WRL parser explicitly
cargo run --bin test_parse
```

*(Note: Native Windows API integration relies on `cfg(target_os = "windows")`. Compiling on Linux defaults to the Sandbox Mock traits.)*

---

## ✍️ Authorship

- **Architecture and Intellectual Design:** Abhinav
- **Implementation:** Jules (Google AI Agent) developed from strict architectural specifications.
- **Language:** Rust
- **Target Platform:** Windows 11 Pro
