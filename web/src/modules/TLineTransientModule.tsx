import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function TLineTransientModule() {
  const [z0, setZ0] = useState(50);
  const [zl, setZl] = useState(100);
  const [zs, setZs] = useState(50);
  const [v0, setV0] = useState(10);
  const [length, setLength] = useState(100); // meters
  const [velocity, setVelocity] = useState(2e8);
  const [bounces, setBounces] = useState(8);

  const td = length / velocity; // one-way delay

  const results = useMemo(() => {
    const gammaL = (zl - z0) / (zl + z0);
    const gammaS = (zs - z0) / (zs + z0);
    const vInit = v0 * z0 / (z0 + zs);

    // Bounce diagram: track voltage at load end
    const times: number[] = [0];
    const vLoad: number[] = [0];
    const vSource: number[] = [vInit];

    let vAtLoad = 0;
    let vAtSource = vInit;
    let currentWave = vInit;

    for (let i = 0; i < bounces; i++) {
      const tArriveLoad = (2 * i + 1) * td;
      const reflected = currentWave * gammaL;
      vAtLoad += currentWave + reflected;
      times.push(tArriveLoad);
      vLoad.push(vAtLoad);

      const tArriveSource = (2 * i + 2) * td;
      const reReflected = reflected * gammaS;
      vAtSource += reflected + reReflected;
      times.push(tArriveSource);
      vSource.push(vAtSource);

      currentWave = reReflected;
    }

    // Steady state
    const vSteady = v0 * zl / (zl + zs);

    // Bounce diagram events
    const bounceEvents: { t: number; x: number; v: number; label: string }[] = [];
    let wave = vInit;
    for (let i = 0; i < bounces; i++) {
      bounceEvents.push({ t: (2 * i + 1) * td, x: length, v: wave, label: `V+=${wave.toFixed(3)}` });
      const ref = wave * gammaL;
      bounceEvents.push({ t: (2 * i + 1) * td, x: length, v: ref, label: `V-=${ref.toFixed(3)}` });
      const reref = ref * gammaS;
      bounceEvents.push({ t: (2 * i + 2) * td, x: 0, v: ref, label: '' });
      wave = reref;
    }

    return { gammaL, gammaS, vInit, vSteady, td, times, vLoad, vSource };
  }, [z0, zl, zs, v0, length, velocity, bounces, td]);

  // Build step waveforms for plotting
  const loadSteps = useMemo(() => {
    const t: number[] = [];
    const v: number[] = [];
    for (let i = 0; i < results.vLoad.length; i++) {
      const tStart = results.times[i * 2 >= results.times.length ? results.times.length - 1 : i];
      if (i === 0) {
        t.push(0); v.push(0);
        t.push(results.td); v.push(0);
      }
    }
    // Simpler: just step through times
    let idx = 0;
    const ts: number[] = [];
    const vs: number[] = [];
    for (let i = 0; i < results.times.length; i++) {
      ts.push(results.times[i]);
      if (i > 0) ts.push(results.times[i]);
      if (i % 2 === 0) {
        vs.push(results.vSource[Math.floor(i / 2)] ?? results.vSource[results.vSource.length - 1]);
        if (i > 0) vs.push(results.vSource[Math.min(Math.floor(i / 2), results.vSource.length - 1)]);
      } else {
        vs.push(results.vLoad[Math.floor(i / 2)] ?? results.vLoad[results.vLoad.length - 1]);
        if (i > 0) vs.push(results.vLoad[Math.min(Math.floor(i / 2), results.vLoad.length - 1)]);
      }
    }
    return { ts, vs };
  }, [results]);

  // Simple load voltage steps
  const loadPlot = useMemo(() => {
    const ts: number[] = [];
    const vs: number[] = [];
    let prevV = 0;
    // Time 0: V=0
    ts.push(0); vs.push(0);
    for (let i = 0; i < results.vLoad.length; i++) {
      const tArr = (2 * i + 1) * results.td;
      ts.push(tArr); vs.push(prevV); // step start
      ts.push(tArr); vs.push(results.vLoad[i]); // step end
      prevV = results.vLoad[i];
    }
    const tEnd = (2 * bounces + 1) * results.td;
    ts.push(tEnd); vs.push(prevV);
    return { ts, vs };
  }, [results, bounces]);

  const srcPlot = useMemo(() => {
    const ts: number[] = [];
    const vs: number[] = [];
    let prevV = 0;
    ts.push(0); vs.push(0);
    for (let i = 0; i < results.vSource.length; i++) {
      const tArr = 2 * i * results.td;
      ts.push(tArr); vs.push(prevV);
      ts.push(tArr); vs.push(results.vSource[i]);
      prevV = results.vSource[i];
    }
    const tEnd = (2 * bounces + 1) * results.td;
    ts.push(tEnd); vs.push(prevV);
    return { ts, vs };
  }, [results, bounces]);

  const fmtTime = (t: number) => {
    if (t < 1e-6) return (t * 1e9).toFixed(1) + ' ns';
    if (t < 1e-3) return (t * 1e6).toFixed(1) + ' μs';
    return (t * 1e3).toFixed(2) + ' ms';
  };

  return (
    <div className="module">
      <h2>Transmission Line Transients</h2>
      <p>Step-response bounce diagram — voltage reflections at source and load.</p>

      <div className="controls">
        <label>Z₀ (Ω): <input type="range" min={10} max={200} step={1} value={z0}
          onChange={e => setZ0(+e.target.value)} /> {z0}</label>
        <label>Z_L (Ω): <input type="range" min={0} max={500} step={1} value={zl}
          onChange={e => setZl(+e.target.value)} /> {zl} {zl === 0 ? '(short)' : zl >= 500 ? '(≈open)' : ''}</label>
        <label>Z_S (Ω): <input type="range" min={0} max={500} step={1} value={zs}
          onChange={e => setZs(+e.target.value)} /> {zs}</label>
        <label>V₀ (V): <input type="range" min={1} max={100} step={1} value={v0}
          onChange={e => setV0(+e.target.value)} /> {v0}</label>
        <label>Length (m): <input type="range" min={1} max={1000} step={1} value={length}
          onChange={e => setLength(+e.target.value)} /> {length}</label>
        <label>Bounces: <input type="range" min={2} max={20} step={1} value={bounces}
          onChange={e => setBounces(+e.target.value)} /> {bounces}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Γ_L</span>
          <span className="value">{results.gammaL.toFixed(4)}</span>
        </div>
        <div className="result-card">
          <span className="label">Γ_S</span>
          <span className="value">{results.gammaS.toFixed(4)}</span>
        </div>
        <div className="result-card">
          <span className="label">V_initial</span>
          <span className="value">{results.vInit.toFixed(3)} V</span>
        </div>
        <div className="result-card">
          <span className="label">V_steady (DC)</span>
          <span className="value">{results.vSteady.toFixed(3)} V</span>
        </div>
        <div className="result-card">
          <span className="label">One-way delay t_d</span>
          <span className="value">{fmtTime(results.td)}</span>
        </div>
      </div>

      <Plot
        data={[
          { x: loadPlot.ts.map(t => t / results.td), y: loadPlot.vs,
            type: 'scatter', mode: 'lines', name: 'V_load',
            line: { color: '#2196F3', width: 2 } },
          { x: srcPlot.ts.map(t => t / results.td), y: srcPlot.vs,
            type: 'scatter', mode: 'lines', name: 'V_source',
            line: { color: '#F44336', width: 2, dash: 'dash' } },
          { x: [0, (2 * bounces + 1)], y: [results.vSteady, results.vSteady],
            type: 'scatter', mode: 'lines', name: 'Steady state',
            line: { color: '#4CAF50', width: 1, dash: 'dot' } },
        ]}
        layout={{
          title: 'Voltage vs Time (Bounce Diagram)',
          xaxis: { title: 't / t_d' },
          yaxis: { title: 'Voltage (V)' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 400,
          legend: { x: 0.65, y: 0.98 },
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Initial wave:</strong> V⁺ = V₀ · Z₀/(Z₀ + Z_S)</p>
        <p><strong>Load reflection:</strong> Γ_L = (Z_L − Z₀)/(Z_L + Z₀)</p>
        <p><strong>Source reflection:</strong> Γ_S = (Z_S − Z₀)/(Z_S + Z₀)</p>
        <p><strong>Steady state:</strong> V_∞ = V₀ · Z_L/(Z_L + Z_S) (voltage divider)</p>
        <p>Each bounce multiplies by Γ_L · Γ_S, converging geometrically to steady state.</p>
      </div>
    </div>
  );
}
