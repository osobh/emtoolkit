import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function MicrowaveOvenBrief() {
  const [power, setPower] = useState(1000);
  const [width, setWidth] = useState(0.35);
  const [depth, setDepth] = useState(0.35);
  const [height, setHeight] = useState(0.25);

  const f = 2.45e9;
  const c = 3e8;
  const lambda = c / f;

  const modes = useMemo(() => {
    const result: { m: number; n: number; p: number; fr: number }[] = [];
    for (let m = 0; m <= 4; m++)
      for (let n = 0; n <= 4; n++)
        for (let p = 0; p <= 4; p++) {
          if (m === 0 && n === 0) continue;
          const fr = (c / 2) * Math.sqrt((m / width) ** 2 + (n / depth) ** 2 + (p / height) ** 2);
          if (fr < 4e9) result.push({ m, n, p, fr });
        }
    result.sort((a, b) => a.fr - b.fr);
    return result.slice(0, 15);
  }, [width, depth, height]);

  // Standing wave pattern (1D simplification)
  const pattern = useMemo(() => {
    const n = 200;
    const xs: number[] = [];
    const field: number[] = [];
    for (let i = 0; i < n; i++) {
      const x = width * i / (n - 1);
      xs.push(x * 100);
      // Sum first few modes near 2.45 GHz
      let e = 0;
      for (const mode of modes) {
        if (Math.abs(mode.fr - f) < 0.3e9) {
          e += Math.sin(mode.m * Math.PI * x / width);
        }
      }
      field.push(e * e); // intensity ∝ E²
    }
    const maxF = Math.max(...field) || 1;
    return { xs, field: field.map(f => f / maxF) };
  }, [modes, width]);

  const penetration = 1.4; // cm in water at 2.45 GHz
  const waterAbsorption = 2 * Math.PI * f * 80 * 8.854e-12 * 0.1; // rough

  return (
    <div className="module">
      <h2>TB3: Microwave Ovens</h2>
      <p>Microwave ovens heat food using 2.45 GHz electromagnetic radiation that excites water molecule rotation.</p>

      <div className="controls">
        <label>Power (W): <input type="range" min={100} max={2000} step={50} value={power}
          onChange={e => setPower(+e.target.value)} /> {power}</label>
        <label>Cavity width (cm): <input type="range" min={20} max={50} step={1}
          value={width * 100} onChange={e => setWidth(+e.target.value / 100)} />
          {(width * 100).toFixed(0)}</label>
        <label>Cavity depth (cm): <input type="range" min={20} max={50} step={1}
          value={depth * 100} onChange={e => setDepth(+e.target.value / 100)} />
          {(depth * 100).toFixed(0)}</label>
        <label>Cavity height (cm): <input type="range" min={15} max={35} step={1}
          value={height * 100} onChange={e => setHeight(+e.target.value / 100)} />
          {(height * 100).toFixed(0)}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Frequency</span>
          <span className="value">2.45 GHz (ISM band)</span>
        </div>
        <div className="result-card">
          <span className="label">Wavelength</span>
          <span className="value">{(lambda * 100).toFixed(1)} cm</span>
        </div>
        <div className="result-card">
          <span className="label">Penetration depth (water)</span>
          <span className="value">~{penetration} cm</span>
        </div>
        <div className="result-card">
          <span className="label">Modes near 2.45 GHz</span>
          <span className="value">{modes.filter(m => Math.abs(m.fr - f) < 0.2e9).length}</span>
        </div>
        <div className="result-card">
          <span className="label">Heat 250mL water</span>
          <span className="value">{(250 * 4.186 * 75 / (power * 0.65) / 60).toFixed(1)} min (20→95°C)</span>
        </div>
      </div>

      <Plot
        data={[{
          x: pattern.xs, y: pattern.field,
          type: 'scatter', mode: 'lines', name: 'Relative heating intensity',
          line: { color: '#F44336', width: 2 },
          fill: 'tozeroy', fillcolor: 'rgba(244,67,54,0.2)',
        }]}
        layout={{
          title: 'Standing Wave Heating Pattern (cross-section)',
          xaxis: { title: 'Position (cm)' },
          yaxis: { title: 'Relative intensity', range: [0, 1.1] },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
          annotations: [{ x: pattern.xs[pattern.xs.length / 2 | 0], y: 0.1,
            text: 'Hot spots at antinodes', showarrow: false, font: { size: 11 } }],
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <h3>Cavity Modes</h3>
      <div style={{ overflowX: 'auto' }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
          <thead><tr style={{ borderBottom: '2px solid #ddd' }}>
            <th style={{ padding: 4 }}>Mode</th><th style={{ padding: 4 }}>f (GHz)</th>
            <th style={{ padding: 4 }}>Near 2.45?</th>
          </tr></thead>
          <tbody>
            {modes.slice(0, 10).map((m, i) => (
              <tr key={i} style={{ background: Math.abs(m.fr - f) < 0.2e9 ? '#FFEBEE' : 'white' }}>
                <td style={{ padding: 4, textAlign: 'center' }}>TE/TM {m.m}{m.n}{m.p}</td>
                <td style={{ padding: 4, textAlign: 'center' }}>{(m.fr / 1e9).toFixed(3)}</td>
                <td style={{ padding: 4, textAlign: 'center' }}>{Math.abs(m.fr - f) < 0.2e9 ? '✅' : ''}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="theory">
        <h3>How Microwave Ovens Work</h3>
        <p><strong>Magnetron</strong> generates 2.45 GHz microwaves. This frequency was chosen because it's in the ISM band (no license needed) and couples well with water molecules.</p>
        <p><strong>Dielectric heating:</strong> Water molecules are polar dipoles. The oscillating E-field forces them to rotate at 2.45 billion times/sec, generating heat through molecular friction.</p>
        <p><strong>Hot spots:</strong> Standing wave patterns in the cavity create nodes (cold) and antinodes (hot). That's why turntables exist — to average out the heating pattern.</p>
      </div>
    </div>
  );
}
