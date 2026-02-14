import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

export function CoaxBFieldModule() {
  const [innerR, setInnerR] = useState(0.002);
  const [outerInnerR, setOuterInnerR] = useState(0.006);
  const [outerOuterR, setOuterOuterR] = useState(0.008);
  const [current, setCurrent] = useState(5.0);

  const data = useMemo(() => {
    try {
      const rMax = outerOuterR * 2;
      return wasm.coaxial_cable_b(innerR, outerInnerR, outerOuterR, current, rMax, 500) as {
        r: number[]; b: number[];
      };
    } catch { return null; }
  }, [innerR, outerInnerR, outerOuterR, current]);

  const bMax = useMemo(() => {
    if (!data) return 0;
    return Math.max(...data.b.map(Math.abs));
  }, [data]);

  return (
    <div className="module">
      <h2>Coaxial Cable B-Field</h2>
      <p>Magnetic field vs. radial distance in a coaxial cable using Ampère's law.</p>

      <div className="controls">
        <label>
          Inner radius a (mm): <input type="range" min={0.5} max={5} step={0.1}
            value={innerR * 1000} onChange={e => setInnerR(+e.target.value / 1000)} />
          {(innerR * 1000).toFixed(1)}
        </label>
        <label>
          Outer inner radius b (mm): <input type="range" min={2} max={15} step={0.5}
            value={outerInnerR * 1000} onChange={e => setOuterInnerR(+e.target.value / 1000)} />
          {(outerInnerR * 1000).toFixed(1)}
        </label>
        <label>
          Outer outer radius c (mm): <input type="range" min={3} max={20} step={0.5}
            value={outerOuterR * 1000} onChange={e => setOuterOuterR(+e.target.value / 1000)} />
          {(outerOuterR * 1000).toFixed(1)}
        </label>
        <label>
          Current I (A): <input type="range" min={0.1} max={50} step={0.1} value={current}
            onChange={e => setCurrent(+e.target.value)} /> {current.toFixed(1)}
        </label>
      </div>

      {data && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">B_max</span>
              <span className="value">{(bMax * 1e3).toFixed(3)} mT</span>
            </div>
            <div className="result-card">
              <span className="label">B_max location</span>
              <span className="value">r = a = {(innerR * 1000).toFixed(1)} mm</span>
            </div>
            <div className="result-card">
              <span className="label">B outside cable</span>
              <span className="value">0 (fields cancel)</span>
            </div>
          </div>

          <Plot
            data={[{
              x: data.r.map(r => r * 1000),
              y: data.b.map(b => b * 1e3),
              type: 'scatter',
              mode: 'lines',
              name: 'B(r)',
              line: { color: '#9C27B0', width: 2 },
              fill: 'tozeroy',
              fillcolor: 'rgba(156,39,176,0.15)',
            }]}
            layout={{
              title: 'B-Field vs. Radial Distance',
              xaxis: { title: 'r (mm)' },
              yaxis: { title: 'B (mT)' },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 400,
              shapes: [
                { type: 'line', x0: innerR * 1000, x1: innerR * 1000, y0: 0, y1: bMax * 1.1e3,
                  line: { color: '#F44336', width: 1, dash: 'dash' } },
                { type: 'line', x0: outerInnerR * 1000, x1: outerInnerR * 1000, y0: 0, y1: bMax * 1.1e3,
                  line: { color: '#FF9800', width: 1, dash: 'dash' } },
                { type: 'line', x0: outerOuterR * 1000, x1: outerOuterR * 1000, y0: 0, y1: bMax * 1.1e3,
                  line: { color: '#4CAF50', width: 1, dash: 'dash' } },
              ],
              annotations: [
                { x: innerR * 1000, y: bMax * 1.05e3, text: 'a', showarrow: false, font: { color: '#F44336' } },
                { x: outerInnerR * 1000, y: bMax * 1.05e3, text: 'b', showarrow: false, font: { color: '#FF9800' } },
                { x: outerOuterR * 1000, y: bMax * 1.05e3, text: 'c', showarrow: false, font: { color: '#4CAF50' } },
              ],
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />
        </>
      )}

      <div className="theory">
        <h3>Regions (Ampère's Law)</h3>
        <p><strong>r &lt; a (inner conductor):</strong> B = μ₀Ir/(2πa²) — linear rise</p>
        <p><strong>a &lt; r &lt; b (dielectric):</strong> B = μ₀I/(2πr) — 1/r decay</p>
        <p><strong>b &lt; r &lt; c (outer conductor):</strong> B decreases as return current cancels</p>
        <p><strong>r &gt; c (outside):</strong> B = 0 — complete field cancellation</p>
      </div>
    </div>
  );
}
