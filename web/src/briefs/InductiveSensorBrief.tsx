import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function InductiveSensorBrief() {
  const [corePos, setCorePos] = useState(50);
  const [excFreq, setExcFreq] = useState(5000);

  const l1Base = 10e-3; // 10 mH
  const l2Base = 10e-3;
  const posNorm = (corePos - 50) / 50; // -1 to +1
  const l1 = l1Base * (1 + 0.8 * posNorm);
  const l2 = l2Base * (1 - 0.8 * posNorm);
  const vDiff = posNorm; // normalized differential output
  const sensitivity = 0.8 * 100 / 50; // %/mm

  const sweepData = useMemo(() => {
    const positions: number[] = [];
    const outputs: number[] = [];
    const l1s: number[] = [];
    const l2s: number[] = [];
    for (let p = 0; p <= 100; p++) {
      const pn = (p - 50) / 50;
      positions.push(p - 50);
      outputs.push(pn * 100);
      l1s.push(l1Base * (1 + 0.8 * pn) * 1e3);
      l2s.push(l2Base * (1 - 0.8 * pn) * 1e3);
    }
    return { positions, outputs, l1s, l2s };
  }, []);

  return (
    <div className="module">
      <h2>TB11: Inductive Sensors (LVDT)</h2>
      <p>Linear Variable Differential Transformers measure displacement by sensing mutual inductance changes as a ferromagnetic core moves through coils.</p>

      <div className="controls">
        <label>Core position (%): <input type="range" min={0} max={100} step={1} value={corePos}
          onChange={e => setCorePos(+e.target.value)} /> {corePos - 50} (centered at 0)</label>
        <label>Excitation freq (Hz): <input type="range" min={1000} max={20000} step={500} value={excFreq}
          onChange={e => setExcFreq(+e.target.value)} /> {excFreq >= 1000 ? (excFreq / 1000).toFixed(1) + ' kHz' : excFreq + ' Hz'}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">L₁ (primary-sec1)</span><span className="value">{(l1 * 1e3).toFixed(2)} mH</span></div>
        <div className="result-card"><span className="label">L₂ (primary-sec2)</span><span className="value">{(l2 * 1e3).toFixed(2)} mH</span></div>
        <div className="result-card"><span className="label">Differential output</span><span className="value">{(vDiff * 100).toFixed(1)}%</span></div>
        <div className="result-card"><span className="label">Sensitivity</span><span className="value">{sensitivity.toFixed(2)} %/mm</span></div>
        <div className="result-card"><span className="label">Phase</span><span className="value">{posNorm >= 0 ? '0°' : '180°'} (indicates direction)</span></div>
      </div>

      <Plot
        data={[
          { x: sweepData.positions, y: sweepData.outputs, type: 'scatter', mode: 'lines',
            name: 'Differential output (%)', line: { color: '#2196F3', width: 2 } },
        ]}
        layout={{
          title: 'LVDT Output vs Displacement',
          xaxis: { title: 'Core position (relative)' },
          yaxis: { title: 'Output (%)' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
          shapes: [{ type: 'line', x0: 0, x1: 0, y0: -100, y1: 100, line: { color: '#ccc', dash: 'dash' } }],
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>How LVDT Works</h3>
        <p><strong>Structure:</strong> One primary coil (center) + two secondary coils (wound in opposition). Movable ferromagnetic core threads through all three.</p>
        <p><strong>At center:</strong> Equal coupling to both secondaries → V₁ = V₂ → differential output = 0.</p>
        <p><strong>Core displaced:</strong> Unequal coupling → |V₁ - V₂| proportional to displacement. Phase indicates direction.</p>
        <p><strong>Advantages:</strong> Frictionless (no contact), infinite resolution, very linear over stroke range, robust, long life.</p>
        <p><strong>Applications:</strong> Industrial automation, hydraulic cylinders, MEMS accelerometers, automotive suspension.</p>
      </div>
    </div>
  );
}
