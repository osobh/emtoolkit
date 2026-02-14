import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function ResonanceModule() {
  const [inductance, setInductance] = useState(1e-3);
  const [capacitance, setCapacitance] = useState(1e-9);
  const [resistance, setResistance] = useState(50);
  const [circuit, setCircuit] = useState<'series' | 'parallel'>('series');

  const f0 = 1 / (2 * Math.PI * Math.sqrt(inductance * capacitance));
  const omega0 = 2 * Math.PI * f0;
  const Q = circuit === 'series'
    ? omega0 * inductance / resistance
    : resistance / (omega0 * inductance);
  const bw = f0 / Q;

  const freqResponse = useMemo(() => {
    const fMin = f0 * 0.1;
    const fMax = f0 * 10;
    const n = 500;
    const freqs: number[] = [];
    const magnitudes: number[] = [];
    const phases: number[] = [];

    for (let i = 0; i < n; i++) {
      const f = fMin * Math.pow(fMax / fMin, i / (n - 1));
      freqs.push(f);
      const w = 2 * Math.PI * f;

      if (circuit === 'series') {
        // Z = R + j(wL - 1/wC)
        const x = w * inductance - 1 / (w * capacitance);
        const zMag = Math.sqrt(resistance * resistance + x * x);
        magnitudes.push(1 / zMag); // |I/V| = 1/|Z|
        phases.push(-Math.atan2(x, resistance) * 180 / Math.PI);
      } else {
        // Y = 1/R + j(wC - 1/wL)
        const b = w * capacitance - 1 / (w * inductance);
        const yMag = Math.sqrt(1 / (resistance * resistance) + b * b);
        magnitudes.push(1 / yMag); // |V/I| = |Z| = 1/|Y|
        phases.push(-Math.atan2(b, 1 / resistance) * 180 / Math.PI);
      }
    }
    // Normalize magnitudes
    const maxM = Math.max(...magnitudes);
    return { freqs, magnitudes: magnitudes.map(m => m / maxM), phases };
  }, [inductance, capacitance, resistance, circuit, f0]);

  return (
    <div className="module">
      <h2>RLC Resonance</h2>
      <p>Explore series and parallel RLC resonance — frequency response, Q factor, and bandwidth.</p>

      <div className="controls">
        <label>Circuit:
          <select value={circuit} onChange={e => setCircuit(e.target.value as 'series' | 'parallel')}>
            <option value="series">Series RLC</option>
            <option value="parallel">Parallel RLC</option>
          </select>
        </label>
        <label>L (μH): <input type="range" min={0.1} max={1000} step={0.1}
          value={inductance * 1e6} onChange={e => setInductance(+e.target.value * 1e-6)} />
          {(inductance * 1e6).toFixed(1)}</label>
        <label>C (pF): <input type="range" min={1} max={10000} step={1}
          value={capacitance * 1e12} onChange={e => setCapacitance(+e.target.value * 1e-12)} />
          {(capacitance * 1e12).toFixed(0)}</label>
        <label>R (Ω): <input type="range" min={1} max={10000} step={1} value={resistance}
          onChange={e => setResistance(+e.target.value)} /> {resistance}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Resonant frequency f₀</span>
          <span className="value">{f0 >= 1e9 ? (f0 / 1e9).toFixed(3) + ' GHz'
            : f0 >= 1e6 ? (f0 / 1e6).toFixed(3) + ' MHz'
            : f0 >= 1e3 ? (f0 / 1e3).toFixed(3) + ' kHz'
            : f0.toFixed(1) + ' Hz'}</span>
        </div>
        <div className="result-card">
          <span className="label">Quality factor Q</span>
          <span className="value">{Q.toFixed(2)}</span>
        </div>
        <div className="result-card">
          <span className="label">Bandwidth (−3 dB)</span>
          <span className="value">{bw >= 1e6 ? (bw / 1e6).toFixed(2) + ' MHz'
            : bw >= 1e3 ? (bw / 1e3).toFixed(2) + ' kHz'
            : bw.toFixed(1) + ' Hz'}</span>
        </div>
        <div className="result-card">
          <span className="label">ω₀</span>
          <span className="value">{omega0.toExponential(3)} rad/s</span>
        </div>
        <div className="result-card">
          <span className="label">X_L at f₀</span>
          <span className="value">{(omega0 * inductance).toFixed(1)} Ω</span>
        </div>
        <div className="result-card">
          <span className="label">X_C at f₀</span>
          <span className="value">{(1 / (omega0 * capacitance)).toFixed(1)} Ω</span>
        </div>
      </div>

      <Plot
        data={[{
          x: freqResponse.freqs.map(f => f >= 1e6 ? f / 1e6 : f / 1e3),
          y: freqResponse.magnitudes.map(m => 20 * Math.log10(m)),
          type: 'scatter', mode: 'lines', name: '|H(f)| dB',
          line: { color: '#2196F3', width: 2 },
        }]}
        layout={{
          title: `${circuit === 'series' ? 'Series' : 'Parallel'} RLC Frequency Response`,
          xaxis: { title: `Frequency (${f0 >= 1e6 ? 'MHz' : 'kHz'})`, type: 'log' },
          yaxis: { title: 'Magnitude (dB)', range: [-40, 3] },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 350,
          shapes: [{ type: 'line', x0: 0, x1: 1e12, y0: -3, y1: -3,
            line: { color: '#F44336', width: 1, dash: 'dash' } }],
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <Plot
        data={[{
          x: freqResponse.freqs.map(f => f >= 1e6 ? f / 1e6 : f / 1e3),
          y: freqResponse.phases,
          type: 'scatter', mode: 'lines', name: 'Phase',
          line: { color: '#4CAF50', width: 2 },
        }]}
        layout={{
          title: 'Phase Response',
          xaxis: { title: `Frequency (${f0 >= 1e6 ? 'MHz' : 'kHz'})`, type: 'log' },
          yaxis: { title: 'Phase (degrees)', range: [-95, 95] },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Resonant frequency:</strong> f₀ = 1/(2π√(LC))</p>
        <p><strong>Series Q:</strong> Q = ω₀L/R = 1/(ω₀CR)</p>
        <p><strong>Parallel Q:</strong> Q = R/(ω₀L) = ω₀CR</p>
        <p><strong>Bandwidth:</strong> BW = f₀/Q</p>
        <p>At resonance, X_L = X_C and impedance is purely resistive.</p>
      </div>
    </div>
  );
}
