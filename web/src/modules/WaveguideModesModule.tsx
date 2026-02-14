import { useState, useMemo } from 'react';

export function WaveguideModesModule() {
  const [a, setA] = useState(22.86); // mm (WR-90)
  const [b, setB] = useState(10.16); // mm
  const [frequency, setFrequency] = useState(10e9);
  const [maxM, setMaxM] = useState(3);
  const [maxN, setMaxN] = useState(3);

  const c = 2.99792458e8;

  const modes = useMemo(() => {
    const result: {
      m: number; n: number; type: string; fc: number;
      propagates: boolean; lambdaG: number | null; vp: number | null; vg: number | null;
    }[] = [];

    for (let m = 0; m <= maxM; m++) {
      for (let n = 0; n <= maxN; n++) {
        if (m === 0 && n === 0) continue;
        // TE modes
        const fcTE = (c / 2) * Math.sqrt((m / (a / 1000)) ** 2 + (n / (b / 1000)) ** 2);
        const propagates = frequency > fcTE;
        let lambdaG = null, vp = null, vg = null;
        if (propagates) {
          const ratio = fcTE / frequency;
          lambdaG = (c / frequency) / Math.sqrt(1 - ratio * ratio);
          vp = c / Math.sqrt(1 - ratio * ratio);
          vg = c * Math.sqrt(1 - ratio * ratio);
        }
        result.push({ m, n, type: 'TE', fc: fcTE, propagates, lambdaG, vp, vg });
      }
    }
    // TM modes (m >= 1, n >= 1)
    for (let m = 1; m <= maxM; m++) {
      for (let n = 1; n <= maxN; n++) {
        const fcTM = (c / 2) * Math.sqrt((m / (a / 1000)) ** 2 + (n / (b / 1000)) ** 2);
        const propagates = frequency > fcTM;
        let lambdaG = null, vp = null, vg = null;
        if (propagates) {
          const ratio = fcTM / frequency;
          lambdaG = (c / frequency) / Math.sqrt(1 - ratio * ratio);
          vp = c / Math.sqrt(1 - ratio * ratio);
          vg = c * Math.sqrt(1 - ratio * ratio);
        }
        result.push({ m, n, type: 'TM', fc: fcTM, propagates, lambdaG, vp, vg });
      }
    }

    result.sort((x, y) => x.fc - y.fc);
    return result;
  }, [a, b, frequency, maxM, maxN]);

  const dominant = modes[0];
  const singleModeMax = modes.length > 1 ? modes[1].fc : Infinity;
  const singleModeBW = singleModeMax - (dominant?.fc ?? 0);

  const presets: Record<string, { a: number; b: number; label: string }> = {
    wr90: { a: 22.86, b: 10.16, label: 'WR-90 (8.2-12.4 GHz)' },
    wr62: { a: 15.80, b: 7.90, label: 'WR-62 (12.4-18 GHz)' },
    wr42: { a: 10.67, b: 4.32, label: 'WR-42 (18-26.5 GHz)' },
    wr28: { a: 7.112, b: 3.556, label: 'WR-28 (26.5-40 GHz)' },
    wr137: { a: 34.85, b: 15.80, label: 'WR-137 (5.85-8.2 GHz)' },
  };

  const fmtFreq = (f: number) => f >= 1e9 ? (f / 1e9).toFixed(3) + ' GHz' : (f / 1e6).toFixed(1) + ' MHz';

  return (
    <div className="module">
      <h2>Waveguide Mode Chart</h2>
      <p>Explore TE and TM modes in a rectangular waveguide — cutoff frequencies, propagation, and velocities.</p>

      <div className="controls">
        <label>Preset:
          <select onChange={e => {
            const p = presets[e.target.value];
            if (p) { setA(p.a); setB(p.b); }
          }}>
            {Object.entries(presets).map(([k, v]) => <option key={k} value={k}>{v.label}</option>)}
          </select>
        </label>
        <label>a (mm): <input type="range" min={1} max={80} step={0.1} value={a}
          onChange={e => setA(+e.target.value)} /> {a.toFixed(2)}</label>
        <label>b (mm): <input type="range" min={1} max={40} step={0.1} value={b}
          onChange={e => setB(+e.target.value)} /> {b.toFixed(2)}</label>
        <label>Operating freq: <input type="range" min={1e9} max={50e9} step={1e8} value={frequency}
          onChange={e => setFrequency(+e.target.value)} /> {fmtFreq(frequency)}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Dominant mode</span>
          <span className="value">{dominant ? `${dominant.type}${dominant.m}${dominant.n}` : 'N/A'}</span>
        </div>
        <div className="result-card">
          <span className="label">Dominant f_c</span>
          <span className="value">{dominant ? fmtFreq(dominant.fc) : 'N/A'}</span>
        </div>
        <div className="result-card">
          <span className="label">Single-mode BW</span>
          <span className="value">{fmtFreq(singleModeBW)}</span>
        </div>
        <div className="result-card">
          <span className="label">Propagating modes</span>
          <span className="value">{modes.filter(m => m.propagates).length}</span>
        </div>
      </div>

      <div style={{ overflowX: 'auto', marginTop: 16 }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #ddd' }}>
              <th style={{ padding: 6 }}>Mode</th>
              <th style={{ padding: 6 }}>f_c</th>
              <th style={{ padding: 6 }}>Status</th>
              <th style={{ padding: 6 }}>λ_g</th>
              <th style={{ padding: 6 }}>v_p / c</th>
              <th style={{ padding: 6 }}>v_g / c</th>
            </tr>
          </thead>
          <tbody>
            {modes.slice(0, 20).map((m, i) => (
              <tr key={i} style={{
                background: m.propagates ? '#E8F5E9' : '#FFEBEE',
                fontWeight: i === 0 ? 'bold' : 'normal',
              }}>
                <td style={{ padding: 6, textAlign: 'center' }}>{m.type}{m.m}{m.n}</td>
                <td style={{ padding: 6, textAlign: 'center' }}>{fmtFreq(m.fc)}</td>
                <td style={{ padding: 6, textAlign: 'center' }}>
                  {m.propagates ? '✅ Propagating' : '❌ Evanescent'}
                </td>
                <td style={{ padding: 6, textAlign: 'center' }}>
                  {m.lambdaG ? (m.lambdaG * 1e3).toFixed(2) + ' mm' : '—'}
                </td>
                <td style={{ padding: 6, textAlign: 'center' }}>
                  {m.vp ? (m.vp / c).toFixed(3) : '—'}
                </td>
                <td style={{ padding: 6, textAlign: 'center' }}>
                  {m.vg ? (m.vg / c).toFixed(3) : '—'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Cutoff:</strong> f_c,mn = (c/2)√((m/a)² + (n/b)²)</p>
        <p><strong>TE modes:</strong> m or n can be 0 (not both). <strong>TM modes:</strong> both m,n ≥ 1</p>
        <p><strong>Guided wavelength:</strong> λ_g = λ / √(1 − (f_c/f)²)</p>
        <p><strong>Phase velocity:</strong> v_p = c / √(1 − (f_c/f)²) &gt; c</p>
        <p><strong>Group velocity:</strong> v_g = c · √(1 − (f_c/f)²) &lt; c</p>
        <p><strong>Note:</strong> v_p · v_g = c² (always!)</p>
      </div>
    </div>
  );
}
