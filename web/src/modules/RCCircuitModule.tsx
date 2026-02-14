import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function RCCircuitModule() {
  const [voltage, setVoltage] = useState(10.0);
  const [resistance, setResistance] = useState(1000.0);
  const [capacitance, setCapacitance] = useState(1e-6);
  const [mode, setMode] = useState<'charge' | 'discharge'>('charge');

  const tau = resistance * capacitance;

  const data = useMemo(() => {
    const tEnd = tau * 6;
    const n = 500;
    const ts: number[] = [];
    const vc: number[] = [];
    const ic: number[] = [];
    for (let i = 0; i < n; i++) {
      const t = tEnd * i / (n - 1);
      ts.push(t);
      if (mode === 'charge') {
        vc.push(voltage * (1 - Math.exp(-t / tau)));
        ic.push((voltage / resistance) * Math.exp(-t / tau));
      } else {
        vc.push(voltage * Math.exp(-t / tau));
        ic.push(-(voltage / resistance) * Math.exp(-t / tau));
      }
    }
    return { ts, vc, ic };
  }, [voltage, resistance, capacitance, mode, tau]);

  const tauFmt = useMemo(() => {
    if (tau < 1e-6) return { val: tau * 1e9, unit: 'ns' };
    if (tau < 1e-3) return { val: tau * 1e6, unit: 'μs' };
    if (tau < 1) return { val: tau * 1e3, unit: 'ms' };
    return { val: tau, unit: 's' };
  }, [tau]);

  return (
    <div className="module">
      <h2>RC Circuit Response</h2>
      <p>Charging and discharging of a capacitor through a resistor.</p>

      <div className="controls">
        <label>Mode:
          <select value={mode} onChange={e => setMode(e.target.value as 'charge' | 'discharge')}>
            <option value="charge">Charging</option>
            <option value="discharge">Discharging</option>
          </select>
        </label>
        <label>V (V): <input type="range" min={1} max={100} step={1} value={voltage}
          onChange={e => setVoltage(+e.target.value)} /> {voltage}</label>
        <label>R (Ω): <input type="range" min={100} max={100000} step={100} value={resistance}
          onChange={e => setResistance(+e.target.value)} /> {resistance >= 1000 ? (resistance / 1000).toFixed(1) + 'k' : resistance}</label>
        <label>C (μF): <input type="range" min={0.01} max={100} step={0.01}
          value={capacitance * 1e6} onChange={e => setCapacitance(+e.target.value * 1e-6)} />
          {(capacitance * 1e6).toFixed(2)}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">τ = RC</span>
          <span className="value">{tauFmt.val.toFixed(2)} {tauFmt.unit}</span>
        </div>
        <div className="result-card">
          <span className="label">Q_max</span>
          <span className="value">{(capacitance * voltage * 1e6).toFixed(2)} μC</span>
        </div>
        <div className="result-card">
          <span className="label">I₀</span>
          <span className="value">{(voltage / resistance * 1e3).toFixed(2)} mA</span>
        </div>
        <div className="result-card">
          <span className="label">Energy stored</span>
          <span className="value">{(0.5 * capacitance * voltage * voltage * 1e6).toFixed(2)} μJ</span>
        </div>
      </div>

      <Plot
        data={[{
          x: data.ts.map(t => t / tau),
          y: data.vc.map(v => v / voltage),
          type: 'scatter', mode: 'lines', name: 'V_C / V₀',
          line: { color: '#2196F3', width: 2 },
        }]}
        layout={{
          title: `Capacitor Voltage (${mode === 'charge' ? 'Charging' : 'Discharging'})`,
          xaxis: { title: 't / τ' },
          yaxis: { title: 'V_C / V₀', range: mode === 'charge' ? [0, 1.1] : [-0.1, 1.1] },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 350,
          annotations: mode === 'charge' ? [
            { x: 1, y: 0.632, text: '63.2% at τ', showarrow: true, arrowhead: 2 },
          ] : [
            { x: 1, y: 0.368, text: '36.8% at τ', showarrow: true, arrowhead: 2 },
          ],
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <Plot
        data={[{
          x: data.ts.map(t => t / tau),
          y: data.ic.map(i => i * 1e3),
          type: 'scatter', mode: 'lines', name: 'i(t)',
          line: { color: '#F44336', width: 2 },
        }]}
        layout={{
          title: 'Current',
          xaxis: { title: 't / τ' },
          yaxis: { title: 'i (mA)' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Charging:</strong> V_C(t) = V₀(1 − e^(−t/τ)), i(t) = (V₀/R)e^(−t/τ)</p>
        <p><strong>Discharging:</strong> V_C(t) = V₀ e^(−t/τ), i(t) = −(V₀/R)e^(−t/τ)</p>
        <p><strong>Time constant:</strong> τ = RC</p>
      </div>
    </div>
  );
}
