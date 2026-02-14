import { useState, useMemo } from 'react';

export function MicrostripModule() {
  const [w, setW] = useState(3.0); // mm
  const [h, setH] = useState(1.6); // mm
  const [t, setT] = useState(0.035); // mm (copper thickness)
  const [epsilonR, setEpsilonR] = useState(4.4); // FR-4
  const [frequency, setFrequency] = useState(2.4e9);
  const [tanDelta, setTanDelta] = useState(0.02);

  const c = 2.99792458e8;

  const results = useMemo(() => {
    const wh = w / h;

    // Effective dielectric constant (Hammerstad-Jensen)
    let epsEff: number;
    if (wh <= 1) {
      epsEff = (epsilonR + 1) / 2 + (epsilonR - 1) / 2 * (
        1 / Math.sqrt(1 + 12 / wh) + 0.04 * (1 - wh) ** 2
      );
    } else {
      epsEff = (epsilonR + 1) / 2 + (epsilonR - 1) / 2 / Math.sqrt(1 + 12 / wh);
    }

    // Characteristic impedance (Hammerstad-Jensen)
    let z0: number;
    if (wh <= 1) {
      z0 = 60 / Math.sqrt(epsEff) * Math.log(8 / wh + wh / 4);
    } else {
      z0 = 120 * Math.PI / (Math.sqrt(epsEff) * (wh + 1.393 + 0.667 * Math.log(wh + 1.444)));
    }

    const vPhase = c / Math.sqrt(epsEff);
    const lambda = vPhase / frequency;
    const lambdaG = lambda;
    const beta = 2 * Math.PI / lambdaG;

    // Conductor loss (approximate)
    const sigma = 5.8e7; // copper
    const mu0 = 4e-7 * Math.PI;
    const skinDepth = 1 / Math.sqrt(Math.PI * frequency * mu0 * sigma);
    const rs = 1 / (sigma * skinDepth);
    const alphaCond = rs / (z0 * w * 1e-3); // Np/m (approximate)

    // Dielectric loss
    const k0 = 2 * Math.PI * frequency / c;
    const alphaDiel = k0 * epsilonR * (epsEff - 1) * tanDelta / (2 * Math.sqrt(epsEff) * (epsilonR - 1));

    const alphaTotal = alphaCond + alphaDiel;
    const lossDbPerCm = alphaTotal * 100 * 20 / Math.log(10) / 100; // dB/cm

    // Wavelength for patch design
    const patchLength = lambda / 2 * 1000; // mm

    return {
      z0, epsEff, vPhase, lambda, beta,
      skinDepth, alphaCond, alphaDiel, alphaTotal, lossDbPerCm,
      patchLength, wh,
    };
  }, [w, h, epsilonR, frequency, tanDelta]);

  const substrates: Record<string, { er: number; tanD: number; label: string }> = {
    fr4: { er: 4.4, tanD: 0.02, label: 'FR-4' },
    rogers4003: { er: 3.55, tanD: 0.0027, label: 'Rogers 4003C' },
    rogers5880: { er: 2.2, tanD: 0.0009, label: 'Rogers 5880' },
    alumina: { er: 9.8, tanD: 0.0001, label: 'Alumina (96%)' },
  };

  return (
    <div className="module">
      <h2>Microstrip Line Calculator</h2>
      <p>Calculate Z₀, effective εᵣ, losses, and guided wavelength for PCB microstrip transmission lines.</p>

      <div className="controls">
        <label>Substrate preset:
          <select onChange={e => {
            const s = substrates[e.target.value];
            if (s) { setEpsilonR(s.er); setTanDelta(s.tanD); }
          }}>
            {Object.entries(substrates).map(([k, v]) => <option key={k} value={k}>{v.label} (εᵣ={v.er})</option>)}
          </select>
        </label>
        <label>Strip width W (mm): <input type="range" min={0.1} max={20} step={0.1} value={w}
          onChange={e => setW(+e.target.value)} /> {w.toFixed(1)}</label>
        <label>Substrate height h (mm): <input type="range" min={0.1} max={5} step={0.1} value={h}
          onChange={e => setH(+e.target.value)} /> {h.toFixed(1)}</label>
        <label>εᵣ: <input type="range" min={1} max={15} step={0.1} value={epsilonR}
          onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR.toFixed(1)}</label>
        <label>tan δ: <input type="range" min={0.0001} max={0.05} step={0.0001} value={tanDelta}
          onChange={e => setTanDelta(+e.target.value)} /> {tanDelta.toFixed(4)}</label>
        <label>Frequency: <input type="range" min={1e8} max={30e9} step={1e8} value={frequency}
          onChange={e => setFrequency(+e.target.value)} />
          {(frequency / 1e9).toFixed(1)} GHz</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Z₀</span>
          <span className="value">{results.z0.toFixed(2)} Ω</span>
        </div>
        <div className="result-card">
          <span className="label">ε_eff</span>
          <span className="value">{results.epsEff.toFixed(3)}</span>
        </div>
        <div className="result-card">
          <span className="label">W/h ratio</span>
          <span className="value">{results.wh.toFixed(3)}</span>
        </div>
        <div className="result-card">
          <span className="label">Phase velocity</span>
          <span className="value">{(results.vPhase / c).toFixed(3)} c</span>
        </div>
        <div className="result-card">
          <span className="label">Guided λ</span>
          <span className="value">{(results.lambda * 1e3).toFixed(2)} mm</span>
        </div>
        <div className="result-card">
          <span className="label">β</span>
          <span className="value">{results.beta.toFixed(2)} rad/m</span>
        </div>
        <div className="result-card">
          <span className="label">Loss</span>
          <span className="value">{results.lossDbPerCm.toFixed(3)} dB/cm</span>
        </div>
        <div className="result-card">
          <span className="label">Skin depth</span>
          <span className="value">{(results.skinDepth * 1e6).toFixed(2)} μm</span>
        </div>
        <div className="result-card">
          <span className="label">λ/2 patch length</span>
          <span className="value">{results.patchLength.toFixed(1)} mm</span>
        </div>
      </div>

      <div className="theory">
        <h3>Hammerstad-Jensen Model</h3>
        <p><strong>W/h ≤ 1:</strong> Z₀ = (60/√ε_eff) ln(8h/W + W/4h)</p>
        <p><strong>W/h &gt; 1:</strong> Z₀ = 120π / [√ε_eff (W/h + 1.393 + 0.667 ln(W/h + 1.444))]</p>
        <p><strong>ε_eff:</strong> accounts for field fringing in air above the strip</p>
        <p><strong>Design tip:</strong> For 50Ω on FR-4 (εᵣ=4.4, h=1.6mm), use W ≈ 3.1mm</p>
      </div>
    </div>
  );
}
