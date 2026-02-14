import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

export function MediumPropertiesModule() {
  const [epsilonR, setEpsilonR] = useState(4.0);
  const [muR, setMuR] = useState(1.0);
  const [conductivity, setConductivity] = useState(0.01);
  const [frequency, setFrequency] = useState(1e9);

  const props = useMemo(
    () => wasm.medium_properties(epsilonR, muR, conductivity, frequency),
    [epsilonR, muR, conductivity, frequency],
  );

  return (
    <div className="module-panel">
      <h2>Wave Propagation in Media</h2>
      <div className="controls">
        <div className="control-group"><label>εᵣ</label><input type="number" value={epsilonR} onChange={e => setEpsilonR(+e.target.value)} step={0.5} min={1} /></div>
        <div className="control-group"><label>μᵣ</label><input type="number" value={muR} onChange={e => setMuR(+e.target.value)} step={0.5} min={1} /></div>
        <div className="control-group"><label>σ (S/m)</label><input type="number" value={conductivity} onChange={e => setConductivity(+e.target.value)} step={0.01} min={0} /></div>
        <div className="control-group"><label>Freq (GHz)</label><input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.001} /></div>
      </div>
      <div className="result-box">
        <h3 style={{ marginTop: 0 }}>Propagation Properties</h3>
        <div className="result-grid">
          <div className="result-item"><span className="result-label">Loss tangent</span><span className="result-value">{props.loss_tangent.toExponential(3)}</span></div>
          <div className="result-item"><span className="result-label">Classification</span><span className="result-value">{props.classification}</span></div>
          <div className="result-item"><span className="result-label">α (Np/m)</span><span className="result-value">{props.alpha.toExponential(3)}</span></div>
          <div className="result-item"><span className="result-label">β (rad/m)</span><span className="result-value">{props.beta.toFixed(3)}</span></div>
          <div className="result-item"><span className="result-label">Skin depth</span><span className="result-value">{props.skin_depth_m != null ? (props.skin_depth_m * 1000).toFixed(3) + ' mm' : '∞'}</span></div>
          <div className="result-item"><span className="result-label">Phase velocity</span><span className="result-value">{(props.phase_velocity / 1e8).toFixed(4)} × 10⁸ m/s</span></div>
          <div className="result-item"><span className="result-label">Wavelength</span><span className="result-value">{(props.wavelength * 100).toFixed(3)} cm</span></div>
          <div className="result-item"><span className="result-label">|η| (Ω)</span><span className="result-value">{props.eta_mag.toFixed(2)}</span></div>
        </div>
      </div>
    </div>
  );
}
