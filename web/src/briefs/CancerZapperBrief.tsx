import { useState } from 'react';

export function CancerZapperBrief() {
  const [frequency, setFrequency] = useState(200);
  const [voltage, setVoltage] = useState(1.0);
  const [tumorSize, setTumorSize] = useState(3.0);

  const c = 3e8;
  const lambda = c / (frequency * 1e3);
  const eField = voltage / (tumorSize / 100);
  const sar = eField * eField * 0.5 / 1040; // rough SAR (σ≈0.5 S/m, ρ≈1040 kg/m³)

  return (
    <div className="module">
      <h2>TB4: EM Cancer Zappers (Tumor Treating Fields)</h2>
      <p>Tumor Treating Fields (TTFields) use alternating electric fields at 100-300 kHz to disrupt cancer cell division.</p>

      <div className="controls">
        <label>Frequency (kHz): <input type="range" min={100} max={300} step={10} value={frequency}
          onChange={e => setFrequency(+e.target.value)} /> {frequency}</label>
        <label>Field intensity (V/cm): <input type="range" min={0.5} max={3.0} step={0.1} value={voltage}
          onChange={e => setVoltage(+e.target.value)} /> {voltage.toFixed(1)}</label>
        <label>Tumor diameter (cm): <input type="range" min={1} max={10} step={0.5} value={tumorSize}
          onChange={e => setTumorSize(+e.target.value)} /> {tumorSize}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Frequency</span>
          <span className="value">{frequency} kHz</span>
        </div>
        <div className="result-card">
          <span className="label">Wavelength</span>
          <span className="value">{(lambda / 1000).toFixed(0)} km</span>
        </div>
        <div className="result-card">
          <span className="label">E-field in tumor</span>
          <span className="value">{voltage.toFixed(1)} V/cm = {(voltage * 100).toFixed(0)} V/m</span>
        </div>
        <div className="result-card">
          <span className="label">Approx. SAR</span>
          <span className="value">{sar.toFixed(2)} W/kg</span>
        </div>
        <div className="result-card">
          <span className="label">Cell size vs λ</span>
          <span className="value">~10μm vs {(lambda / 1000).toFixed(0)} km (quasi-static)</span>
        </div>
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: '#E8F5E9', borderRadius: 8 }}>
        <h4>How TTFields Disrupt Mitosis</h4>
        <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap' }}>
          <div style={{ flex: 1, minWidth: 200 }}>
            <p style={{ fontSize: 14 }}><strong>1. Alignment phase:</strong> During metaphase, the uniform E-field aligns polar tubulin proteins, preventing proper spindle formation.</p>
          </div>
          <div style={{ flex: 1, minWidth: 200 }}>
            <p style={{ fontSize: 14 }}><strong>2. Dielectrophoresis:</strong> During cytokinesis (cell pinching), the hourglass shape creates non-uniform fields that push organelles toward the cleavage furrow, damaging the cell.</p>
          </div>
          <div style={{ flex: 1, minWidth: 200 }}>
            <p style={{ fontSize: 14 }}><strong>3. Frequency tuning:</strong> Optimal frequency depends on cell size. 200 kHz works for glioblastoma; 150 kHz for mesothelioma.</p>
          </div>
        </div>
      </div>

      <div className="theory">
        <h3>Physics of TTFields</h3>
        <p><strong>Quasi-static regime:</strong> At 200 kHz, λ ≈ 1.5 km. Cells (10μm) are 10⁸ times smaller — the field is effectively uniform across each cell.</p>
        <p><strong>Force on dipoles:</strong> F = (p·∇)E. In non-uniform fields, polar molecules experience dielectrophoretic force toward regions of higher field intensity.</p>
        <p><strong>Clinical use:</strong> Optune® (Novocure) — FDA-approved for glioblastoma. Patients wear electrode arrays on shaved scalp, delivering TTFields continuously.</p>
        <p><strong>Key insight:</strong> TTFields exploit the fact that dividing cells have unique electrical vulnerabilities (polar proteins, changing geometry) that quiescent cells don't.</p>
      </div>
    </div>
  );
}
