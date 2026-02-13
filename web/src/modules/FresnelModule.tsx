import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function FresnelModule() {
  const [er1, setEr1] = useState(1.0);
  const [er2, setEr2] = useState(4.0);
  const [thetaI, setThetaI] = useState(30);

  const vsAngle = useMemo(() => wasm.fresnel_vs_angle(er1, er2, 200), [er1, er2]);
  const atAngle = useMemo(() => wasm.fresnel_oblique(er1, er2, thetaI), [er1, er2, thetaI]);

  return (
    <div className="module-panel">
      <h2>Fresnel Coefficients</h2>
      <div className="controls">
        <div className="control-group">
          <label>εᵣ₁</label>
          <input type="number" value={er1} onChange={e => setEr1(+e.target.value)} step={0.5} min={1} />
        </div>
        <div className="control-group">
          <label>εᵣ₂</label>
          <input type="number" value={er2} onChange={e => setEr2(+e.target.value)} step={0.5} min={1} />
        </div>
        <div className="control-group">
          <label>θᵢ (degrees)</label>
          <input type="range" min={0} max={89} step={1} value={thetaI}
            onChange={e => setThetaI(+e.target.value)} />
          <span>{thetaI}°</span>
        </div>
      </div>

      <Plot
        data={[
          { x: vsAngle.angles_deg, y: vsAngle.gamma_perp, mode: 'lines', line: { color: '#2196f3', width: 2 }, name: 'Γ⊥ (TE)' },
          { x: vsAngle.angles_deg, y: vsAngle.gamma_par, mode: 'lines', line: { color: '#e63946', width: 2 }, name: 'Γ∥ (TM)' },
          { x: [thetaI, thetaI], y: [-1, 1], mode: 'lines', line: { color: '#888', width: 1, dash: 'dash' }, showlegend: false },
        ]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Angle of Incidence (°)' }, range: [0, 90] },
          yaxis: { title: { text: 'Reflection Coefficient' }, range: [-1.1, 1.1] },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          legend: { x: 0.02, y: 0.98 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />

      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">θₜ</span><span className="result-value">{atAngle.theta_t_deg != null ? atAngle.theta_t_deg.toFixed(1) + '°' : 'TIR'}</span></div>
          <div className="result-item"><span className="result-label">TIR?</span><span className="result-value">{atAngle.is_tir ? 'Yes' : 'No'}</span></div>
          <div className="result-item"><span className="result-label">Brewster angle</span><span className="result-value">{atAngle.brewster_angle_deg.toFixed(1)}°</span></div>
          <div className="result-item"><span className="result-label">Critical angle</span><span className="result-value">{atAngle.critical_angle_deg != null ? atAngle.critical_angle_deg.toFixed(1) + '°' : 'N/A'}</span></div>
          <div className="result-item"><span className="result-label">Γ⊥</span><span className="result-value">{atAngle.gamma_perp != null ? atAngle.gamma_perp.toFixed(4) : '—'}</span></div>
          <div className="result-item"><span className="result-label">Γ∥</span><span className="result-value">{atAngle.gamma_par != null ? atAngle.gamma_par.toFixed(4) : '—'}</span></div>
        </div>
      </div>
    </div>
  );
}
