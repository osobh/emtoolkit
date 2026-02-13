import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function SmithChartModule() {
  const [zlRe, setZlRe] = useState(25);
  const [zlIm, setZlIm] = useState(50);
  const [z0, setZ0] = useState(50);
  const [traceLength, setTraceLength] = useState(Math.PI);

  const zNormRe = zlRe / z0;
  const zNormIm = zlIm / z0;

  const point = useMemo(() => wasm.smith_chart_point(zNormRe, zNormIm), [zNormRe, zNormIm]);

  const swrCircle = useMemo(() => {
    return wasm.smith_chart_swr_circle(point.gamma_mag, 200);
  }, [point.gamma_mag]);

  const trace = useMemo(() => {
    return wasm.smith_chart_trace(zNormRe, zNormIm, traceLength, 200);
  }, [zNormRe, zNormIm, traceLength]);

  // Unit circle
  const unitCircle = useMemo(() => {
    const t = Array.from({ length: 201 }, (_, i) => (i / 200) * 2 * Math.PI);
    return { x: t.map(a => Math.cos(a)), y: t.map(a => Math.sin(a)) };
  }, []);

  return (
    <div className="module-panel">
      <h2>Smith Chart Explorer</h2>
      <div className="controls">
        <div className="control-group">
          <label>Z_L Real (Ω)</label>
          <input type="number" value={zlRe} onChange={e => setZlRe(+e.target.value)} step={5} />
        </div>
        <div className="control-group">
          <label>Z_L Imag (Ω)</label>
          <input type="number" value={zlIm} onChange={e => setZlIm(+e.target.value)} step={5} />
        </div>
        <div className="control-group">
          <label>Z₀ (Ω)</label>
          <input type="number" value={z0} onChange={e => setZ0(+e.target.value)} step={5} min={1} />
        </div>
        <div className="control-group">
          <label>Trace length (βl rad)</label>
          <input type="range" min={0} max={Math.PI * 2} step={0.05} value={traceLength}
            onChange={e => setTraceLength(+e.target.value)} />
          <span>{(traceLength / Math.PI).toFixed(2)}π</span>
        </div>
      </div>

      <Plot
        data={[
          { x: unitCircle.x, y: unitCircle.y, mode: 'lines', line: { color: '#ccc', width: 1 }, showlegend: false, hoverinfo: 'skip' },
          { x: swrCircle.x, y: swrCircle.y, mode: 'lines', line: { color: '#4ecdc4', width: 2, dash: 'dot' }, name: `SWR=${point.vswr.toFixed(2)}` },
          { x: trace.gamma_re, y: trace.gamma_im, mode: 'lines', line: { color: '#ff6b6b', width: 2 }, name: 'Toward Generator' },
          { x: [point.gamma_re], y: [point.gamma_im], mode: 'markers', marker: { size: 12, color: '#e63946' }, name: `Γ = ${point.gamma_mag.toFixed(3)}∠${point.gamma_phase_deg.toFixed(1)}°` },
        ]}
        layout={{
          width: 600, height: 600,
          xaxis: { range: [-1.2, 1.2], scaleanchor: 'y', title: { text: 'Re(Γ)' } },
          yaxis: { range: [-1.2, 1.2], title: { text: 'Im(Γ)' } },
          margin: { t: 30, b: 50, l: 50, r: 30 },
          legend: { x: 0, y: -0.15, orientation: 'h' },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />

      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">Γ magnitude</span><span className="result-value">{point.gamma_mag.toFixed(4)}</span></div>
          <div className="result-item"><span className="result-label">Γ phase</span><span className="result-value">{point.gamma_phase_deg.toFixed(1)}°</span></div>
          <div className="result-item"><span className="result-label">VSWR</span><span className="result-value">{point.vswr.toFixed(3)}</span></div>
          <div className="result-item"><span className="result-label">Return Loss</span><span className="result-value">{point.return_loss_db.toFixed(2)} dB</span></div>
          <div className="result-item"><span className="result-label">r (norm)</span><span className="result-value">{point.r.toFixed(4)}</span></div>
          <div className="result-item"><span className="result-label">x (norm)</span><span className="result-value">{point.x.toFixed(4)}</span></div>
        </div>
      </div>
    </div>
  );
}
