import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface RLResult {
  t: number[];
  current: number[];
  time_constant: number;
  i_final: number;
}

export function RLCircuitModule() {
  const [voltage, setVoltage] = useState(10.0);
  const [resistance, setResistance] = useState(100.0);
  const [inductance, setInductance] = useState(0.01);

  const result: RLResult | null = useMemo(() => {
    try {
      const tau = inductance / resistance;
      return wasm.rl_step(voltage, resistance, inductance, tau * 6, 500) as RLResult;
    } catch { return null; }
  }, [voltage, resistance, inductance]);

  const tauUnit = useMemo(() => {
    if (!result) return { val: 0, unit: 's' };
    const t = result.time_constant;
    if (t < 1e-6) return { val: t * 1e9, unit: 'ns' };
    if (t < 1e-3) return { val: t * 1e6, unit: 'μs' };
    if (t < 1) return { val: t * 1e3, unit: 'ms' };
    return { val: t, unit: 's' };
  }, [result]);

  return (
    <div className="module">
      <h2>RL Circuit Step Response</h2>
      <p>Visualize current rise in a series RL circuit when a DC voltage is applied at t = 0.</p>

      <div className="controls">
        <label>V (Volts): <input type="range" min={1} max={100} step={1} value={voltage}
          onChange={e => setVoltage(+e.target.value)} /> {voltage.toFixed(0)}</label>
        <label>R (Ω): <input type="range" min={1} max={10000} step={1} value={resistance}
          onChange={e => setResistance(+e.target.value)} /> {resistance}</label>
        <label>L (mH): <input type="range" min={0.01} max={100} step={0.01}
          value={inductance * 1000} onChange={e => setInductance(+e.target.value / 1000)} />
          {(inductance * 1000).toFixed(2)}</label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">τ = L/R</span>
              <span className="value">{tauUnit.val.toFixed(2)} {tauUnit.unit}</span>
            </div>
            <div className="result-card">
              <span className="label">I_final = V/R</span>
              <span className="value">{result.i_final < 0.01
                ? (result.i_final * 1e3).toFixed(2) + ' mA'
                : result.i_final.toFixed(3) + ' A'}</span>
            </div>
            <div className="result-card">
              <span className="label">At t = τ</span>
              <span className="value">{(result.i_final * 0.632).toFixed(3)} A (63.2%)</span>
            </div>
            <div className="result-card">
              <span className="label">At t = 5τ</span>
              <span className="value">{(result.i_final * 0.9933).toFixed(3)} A (99.3%)</span>
            </div>
          </div>

          <Plot
            data={[
              {
                x: result.t.map(t => t / result.time_constant),
                y: result.current.map(i => i / result.i_final),
                type: 'scatter', mode: 'lines', name: 'i(t)/I_final',
                line: { color: '#2196F3', width: 2 },
              },
              {
                x: [1, 1], y: [0, 0.632],
                type: 'scatter', mode: 'lines', name: 'τ',
                line: { color: '#F44336', width: 1, dash: 'dash' },
                showlegend: false,
              },
              {
                x: [0, 1], y: [0.632, 0.632],
                type: 'scatter', mode: 'lines',
                line: { color: '#F44336', width: 1, dash: 'dash' },
                showlegend: false,
              },
            ]}
            layout={{
              title: 'RL Step Response',
              xaxis: { title: 't / τ' },
              yaxis: { title: 'i(t) / I_final', range: [0, 1.1] },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 400,
              annotations: [
                { x: 1, y: 0.632, text: '63.2% at t = τ', showarrow: true, arrowhead: 2 },
              ],
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />

          <Plot
            data={[{
              x: result.t.map(t => {
                const tau = result.time_constant;
                if (tau < 1e-3) return t * 1e6;
                if (tau < 1) return t * 1e3;
                return t;
              }),
              y: result.current.map(i => i * 1e3),
              type: 'scatter', mode: 'lines', name: 'i(t)',
              line: { color: '#4CAF50', width: 2 },
            }]}
            layout={{
              title: 'Absolute Current',
              xaxis: { title: `t (${result.time_constant < 1e-3 ? 'μs' : result.time_constant < 1 ? 'ms' : 's'})` },
              yaxis: { title: 'i (mA)' },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 350,
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Differential equation:</strong> V = L(di/dt) + Ri</p>
        <p><strong>Solution:</strong> i(t) = (V/R)(1 − e^(−t/τ))</p>
        <p><strong>Time constant:</strong> τ = L/R</p>
        <p>At t = τ: 63.2%, at t = 3τ: 95%, at t = 5τ: 99.3% of final value.</p>
      </div>
    </div>
  );
}
