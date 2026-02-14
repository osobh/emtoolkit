import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function ImpedanceModule() {
  const [zlRe, setZlRe] = useState(75);
  const [zlIm, setZlIm] = useState(25);
  const [z0, setZ0] = useState(50);

  const gamma = useMemo(() => wasm.reflection_coefficient(zlRe, zlIm, z0), [zlRe, zlIm, z0]);

  // Sweep beta*l from 0 to 2pi to show Z_in variation
  const sweep = useMemo(() => {
    const N = 200;
    const betaLs: number[] = [];
    const zinRe: number[] = [];
    const zinIm: number[] = [];
    for (let i = 0; i <= N; i++) {
      const bl = (i / N) * 2 * Math.PI;
      betaLs.push(bl);
      const zin = wasm.input_impedance_lossless(zlRe, zlIm, z0, bl);
      zinRe.push(zin.re);
      zinIm.push(zin.im);
    }
    return { betaLs, zinRe, zinIm };
  }, [zlRe, zlIm, z0]);

  return (
    <div className="module-panel">
      <h2>Reflection Coefficient &amp; Input Impedance</h2>
      <div className="controls">
        <div className="control-group"><label>Z_L Real (Ω)</label><input type="number" value={zlRe} onChange={e => setZlRe(+e.target.value)} step={5} /></div>
        <div className="control-group"><label>Z_L Imag (Ω)</label><input type="number" value={zlIm} onChange={e => setZlIm(+e.target.value)} step={5} /></div>
        <div className="control-group"><label>Z₀ (Ω)</label><input type="number" value={z0} onChange={e => setZ0(+e.target.value)} step={5} min={1} /></div>
      </div>

      <div className="result-box">
        <h3 style={{ marginTop: 0 }}>Reflection Coefficient</h3>
        <div className="result-grid">
          <div className="result-item"><span className="result-label">|Γ|</span><span className="result-value">{gamma.magnitude.toFixed(4)}</span></div>
          <div className="result-item"><span className="result-label">∠Γ</span><span className="result-value">{gamma.phase_deg.toFixed(2)}°</span></div>
          <div className="result-item"><span className="result-label">VSWR</span><span className="result-value">{gamma.vswr.toFixed(3)}</span></div>
          <div className="result-item"><span className="result-label">Γ (re)</span><span className="result-value">{gamma.re.toFixed(4)}</span></div>
          <div className="result-item"><span className="result-label">Γ (im)</span><span className="result-value">{gamma.im.toFixed(4)}</span></div>
        </div>
      </div>

      <Plot
        data={[
          { x: sweep.betaLs.map(b => b / Math.PI), y: sweep.zinRe, mode: 'lines', line: { color: '#2196f3', width: 2 }, name: 'Re{Z_in}' },
          { x: sweep.betaLs.map(b => b / Math.PI), y: sweep.zinIm, mode: 'lines', line: { color: '#e63946', width: 2 }, name: 'Im{Z_in}' },
          { x: sweep.betaLs.map(b => b / Math.PI), y: sweep.betaLs.map(() => z0), mode: 'lines', line: { color: '#888', width: 1, dash: 'dash' }, name: 'Z₀', showlegend: true },
        ]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'βl / π' } },
          yaxis: { title: { text: 'Impedance (Ω)' } },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          legend: { x: 0.7, y: 0.98 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
    </div>
  );
}
