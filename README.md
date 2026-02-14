# ⚡ EM Toolkit

**Interactive Electromagnetics Education Platform**

A comprehensive, browser-based toolkit for learning and exploring electromagnetics — from electrostatics to antenna design. Built with Rust + WebAssembly for computational accuracy and React for interactive visualization.

[![Tests](https://img.shields.io/badge/tests-477%20passing-brightgreen)]()
[![WASM](https://img.shields.io/badge/WASM-237KB%20(93KB%20gzip)-blue)]()
[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange)]()
[![React](https://img.shields.io/badge/React-19-61dafb)]()

---

## 🎯 What Is This?

EM Toolkit is a standalone interactive platform covering the full spectrum of undergraduate/graduate electromagnetics. Each module provides real-time computation with adjustable parameters, instant visualization, and physical insight — no MATLAB license required.

**25 interactive simulation modules** across 8 topic areas, powered by **10 Rust crates** compiled to WebAssembly.

---

## 📚 Modules

### Chapter 1: Foundations
| Module | Description |
|--------|-------------|
| **Constants Explorer** | ε₀, μ₀, c, η₀ with unit conversions |
| **Coordinate Systems** | Cartesian ↔ Cylindrical ↔ Spherical transforms |
| **Medium Properties** | σ, ε, μ characterization with loss tangent |
| **Charge Relaxation** | Time-constant visualization for lossy media |

### Chapter 2: Vector Analysis
| Module | Description |
|--------|-------------|
| **Vector Field Visualizer** | 2D quiver plots with divergence/curl overlays |
| **Gradient Explorer** | Scalar field → gradient vector visualization |

### Chapter 3: Electrostatics
| Module | Description |
|--------|-------------|
| **Electrostatics Lab** | Coulomb's law, E-field from point charges |
| **Method of Images** | Charges near conductors with image solutions |

### Chapter 4: Magnetostatics
| Module | Description |
|--------|-------------|
| **Magnetostatics Lab** | Biot-Savart law, B-field from wire configurations |
| **Wire Force Calculator** | Force between parallel current-carrying wires |

### Chapter 5: Time-Varying Fields
| Module | Description |
|--------|-------------|
| **Faraday's Law** | EMF from time-varying magnetic flux |
| **Displacement Current** | Maxwell's correction to Ampère's law |

### Chapter 6: Wave Propagation
| Module | Description |
|--------|-------------|
| **Traveling Wave** | E(z,t) animation with phase velocity |
| **Waveform Analyzer** | λ, f, k, ω relationships |
| **Polarization** | Linear, circular, elliptical visualization |
| **Phase Comparison** | Superposition of two waves |
| **Fresnel Coefficients** | Reflection/transmission at interfaces (TE/TM) |

### Chapter 7: Transmission Lines
| Module | Description |
|--------|-------------|
| **Standing Wave** | VSWR patterns on terminated lines |
| **Impedance Transform** | Z(d) along a transmission line |
| **Smith Chart** | Interactive impedance matching |
| **Matching Network** | Single/double stub tuner design |
| **Coaxial Line** | Z₀, velocity factor, attenuation |

### Chapter 8: Antennas & Link Budget
| Module | Description |
|--------|-------------|
| **Dipole Antenna** | Radiation pattern for λ/2 and short dipoles |
| **Array Factor** | N-element uniform linear array patterns |
| **Link Budget** | Friis equation with path loss and margins |

---

## 🏗️ Architecture

```
em-toolkit/
├── crates/
│   ├── em-core/           # Physical constants, complex math, coordinate transforms
│   ├── em-waves/          # Plane waves, polarization, Fresnel equations
│   ├── em-transmission/   # T-line theory, Smith chart, impedance matching
│   ├── em-vectors/        # Vector calculus, div, curl, gradient
│   ├── em-electrostatics/ # Coulomb's law, Gauss's law, method of images
│   ├── em-magnetostatics/ # Biot-Savart, Ampère's law, wire forces
│   ├── em-timevarying/    # Faraday's law, displacement current
│   ├── em-propagation/    # Wave propagation in lossy/lossless media
│   ├── em-antennas/       # Dipoles, arrays, Friis equation, link budget
│   └── em-wasm/           # WebAssembly bindings (40+ exported functions)
├── web/                   # React 19 + TypeScript + Vite + Plotly.js
│   └── src/modules/       # 25 interactive simulation modules
└── Cargo.toml             # Workspace root
```

### Crate Dependency Graph

```
em-wasm ─┬─ em-antennas ──── em-core
         ├─ em-propagation ── em-core
         ├─ em-timevarying ── em-core
         ├─ em-magnetostatics ── em-core
         ├─ em-electrostatics ── em-core
         ├─ em-vectors ────── em-core
         ├─ em-transmission ── em-core
         ├─ em-waves ──────── em-core
         └─ em-core
```

---

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) 1.93+ (2024 Edition)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) 18+

### Build & Run

```bash
# Clone
git clone git@github.com:osobh/emtoolkit.git
cd emtoolkit

# Build WASM
cd crates/em-wasm
wasm-pack build --target web --release
cd ../..

# Copy WASM to frontend
cp crates/em-wasm/pkg/em_wasm_bg.wasm web/public/wasm/
cp crates/em-wasm/pkg/em_wasm.js web/src/lib/

# Install frontend deps & run
cd web
npm install
npm run dev
```

Open **http://localhost:5173** in your browser.

### Run Tests

```bash
# All 477 Rust tests
cargo test

# Individual crate
cargo test -p em-core
cargo test -p em-waves
cargo test -p em-transmission
# ... etc
```

---

## 🧪 Test Coverage

| Crate | Tests | Key Coverage |
|-------|------:|-------------|
| `em-core` | 104 | Complex arithmetic, coordinate transforms, unit conversions |
| `em-waves` | 42 | Plane wave propagation, polarization, Fresnel coefficients |
| `em-transmission` | 91 | Smith chart, VSWR, impedance matching, stub tuners |
| `em-vectors` | 47 | Gradient, divergence, curl in all coordinate systems |
| `em-electrostatics` | 26 | Coulomb fields, Gauss's law, method of images |
| `em-magnetostatics` | 55 | Biot-Savart, Ampère's law, solenoid/toroid fields |
| `em-timevarying` | 37 | Faraday's law, displacement current, Maxwell's equations |
| `em-propagation` | 41 | Skin depth, attenuation, phase velocity in lossy media |
| `em-antennas` | 34 | Dipole patterns, array factor, Friis equation |
| **Total** | **477** | |

All tests validated against textbook references (Ulaby, Griffiths, Balanis).

---

## 📦 WASM Binary

- **Release size:** 237 KB (93 KB gzip)
- **40+ exported functions** covering all 9 domain crates
- Zero dependencies beyond `wasm-bindgen` and `serde`

---

## 🛠️ Tech Stack

| Layer | Technology |
|-------|-----------|
| **Compute Engine** | Rust 2024 Edition |
| **WASM Bindings** | wasm-bindgen + serde-wasm-bindgen |
| **Frontend** | React 19 + TypeScript 5 |
| **Build** | Vite 7 |
| **Visualization** | Plotly.js (react-plotly.js) |
| **Styling** | Tailwind CSS v4 |

---

## 🗺️ Roadmap

- [ ] 31 additional simulation modules (56 total planned)
- [ ] 17 interactive tech briefs with worked examples
- [ ] Tauri desktop app for native performance
- [ ] PDF/PNG export of plots and calculations
- [ ] Dark mode
- [ ] Mobile-responsive layouts

---

## 📖 References

- F.T. Ulaby, *Fundamentals of Applied Electromagnetics*, 8th Edition
- D.J. Griffiths, *Introduction to Electrodynamics*, 4th Edition
- C.A. Balanis, *Antenna Theory*, 4th Edition
- S.J. Orfanidis, *Electromagnetic Waves and Antennas*

---

## 📄 License

MIT

---

*Built with Rust 🦀 + WebAssembly for computational accuracy at browser speed.*
