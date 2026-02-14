import { useState } from 'react';

export function CTScanBrief() {
  const [voltage, setVoltage] = useState(120);
  const [projections, setProjections] = useState(360);
  const [detectors, setDetectors] = useState(900);

  const maxEnergy = voltage; // keV
  const minWavelength = 1240 / (voltage * 1000) * 1e9; // pm
  const slicePixels = projections * detectors;
  const dose = voltage * voltage * 0.001; // rough mGy approximation

  return (
    <div className="module">
      <h2>TB6: X-Ray Computed Tomography</h2>
      <p>CT scanning uses X-ray attenuation measurements from multiple angles to reconstruct cross-sectional images via the Radon transform.</p>

      <div className="controls">
        <label>Tube voltage (kVp): <input type="range" min={80} max={150} step={10} value={voltage}
          onChange={e => setVoltage(+e.target.value)} /> {voltage}</label>
        <label>Projections/rotation: <input type="range" min={90} max={1080} step={90} value={projections}
          onChange={e => setProjections(+e.target.value)} /> {projections}</label>
        <label>Detectors per row: <input type="range" min={300} max={2000} step={100} value={detectors}
          onChange={e => setDetectors(+e.target.value)} /> {detectors}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Max photon energy</span><span className="value">{maxEnergy} keV</span></div>
        <div className="result-card"><span className="label">Min wavelength</span><span className="value">{minWavelength.toFixed(1)} pm</span></div>
        <div className="result-card"><span className="label">Data points/slice</span><span className="value">{(slicePixels / 1000).toFixed(0)}k</span></div>
        <div className="result-card"><span className="label">Approx. dose</span><span className="value">~{dose.toFixed(1)} mGy</span></div>
        <div className="result-card"><span className="label">Frequency</span><span className="value">{(maxEnergy * 1e3 * 1.6e-19 / 6.626e-34 / 1e18).toFixed(1)} × 10¹⁸ Hz</span></div>
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: '#FFF3E0', borderRadius: 8 }}>
        <h4>Radon Transform & Back-Projection</h4>
        <p style={{ fontSize: 14 }}><strong>Forward problem:</strong> Each X-ray measurement gives the line integral of attenuation: p(s,θ) = ∫μ(x,y)dl along the ray.</p>
        <p style={{ fontSize: 14 }}><strong>Sinogram:</strong> Collection of all projections at all angles forms the sinogram — a 2D function p(s,θ).</p>
        <p style={{ fontSize: 14 }}><strong>Reconstruction:</strong> Filtered back-projection (FBP) or iterative algorithms (ART, SART) invert the Radon transform to recover μ(x,y).</p>
        <p style={{ fontSize: 14 }}><strong>Hounsfield units:</strong> HU = 1000 × (μ − μ_water) / μ_water. Air = −1000, water = 0, bone = +1000.</p>
      </div>

      <div className="theory">
        <h3>EM in CT Scanning</h3>
        <p><strong>X-ray generation:</strong> Bremsstrahlung — electrons decelerated by a tungsten target emit broadband X-rays.</p>
        <p><strong>Attenuation:</strong> I = I₀ exp(−μx) — Beer-Lambert law. Different tissues have different μ values.</p>
        <p><strong>Photoelectric effect + Compton scattering</strong> dominate at diagnostic energies (30-150 keV).</p>
      </div>
    </div>
  );
}
