import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function SolarCellBrief() {
  const [isc, setIsc] = useState(8.0);
  const [voc, setVoc] = useState(0.65);
  const [n, setN] = useState(1.3);
  const [irradiance, setIrradiance] = useState(1000);
  const [temp, setTemp] = useState(25);

  const vt = 0.02585 * (273.15 + temp) / 298.15;

  const ivData = useMemo(() => {
    const iscAdj = isc * irradiance / 1000;
    const vocAdj = voc - 0.002 * (temp - 25);
    const i0 = iscAdj / (Math.exp(vocAdj / (n * vt)) - 1);

    const vs: number[] = [];
    const is: number[] = [];
    const ps: number[] = [];
    let pmax = 0, vmp = 0, imp = 0;

    for (let vi = 0; vi <= vocAdj * 1.05; vi += vocAdj / 200) {
      const current = iscAdj - i0 * (Math.exp(vi / (n * vt)) - 1);
      if (current < 0) break;
      vs.push(vi);
      is.push(current);
      const p = vi * current;
      ps.push(p);
      if (p > pmax) { pmax = p; vmp = vi; imp = current; }
    }

    const ff = pmax / (iscAdj * vocAdj);
    const efficiency = pmax / (irradiance / 1000 * 0.016); // 0.016 m² typical cell

    return { vs, is, ps, pmax, vmp, imp, ff, efficiency, iscAdj, vocAdj };
  }, [isc, voc, n, irradiance, temp, vt]);

  return (
    <div className="module">
      <h2>TB2: Solar Cells</h2>
      <p>Photovoltaic cells convert sunlight directly to electricity using the photovoltaic effect in semiconductor p-n junctions.</p>

      <div className="controls">
        <label>I_sc (A): <input type="range" min={1} max={15} step={0.1} value={isc}
          onChange={e => setIsc(+e.target.value)} /> {isc.toFixed(1)}</label>
        <label>V_oc (V): <input type="range" min={0.3} max={1.2} step={0.01} value={voc}
          onChange={e => setVoc(+e.target.value)} /> {voc.toFixed(2)}</label>
        <label>Ideality factor n: <input type="range" min={1} max={2} step={0.1} value={n}
          onChange={e => setN(+e.target.value)} /> {n.toFixed(1)}</label>
        <label>Irradiance (W/m²): <input type="range" min={100} max={1200} step={50} value={irradiance}
          onChange={e => setIrradiance(+e.target.value)} /> {irradiance}</label>
        <label>Temperature (°C): <input type="range" min={0} max={80} step={1} value={temp}
          onChange={e => setTemp(+e.target.value)} /> {temp}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">P_max (MPP)</span>
          <span className="value">{ivData.pmax.toFixed(2)} W</span>
        </div>
        <div className="result-card">
          <span className="label">V_mp</span>
          <span className="value">{ivData.vmp.toFixed(3)} V</span>
        </div>
        <div className="result-card">
          <span className="label">I_mp</span>
          <span className="value">{ivData.imp.toFixed(2)} A</span>
        </div>
        <div className="result-card">
          <span className="label">Fill Factor</span>
          <span className="value">{(ivData.ff * 100).toFixed(1)}%</span>
        </div>
        <div className="result-card">
          <span className="label">I_sc (adjusted)</span>
          <span className="value">{ivData.iscAdj.toFixed(2)} A</span>
        </div>
        <div className="result-card">
          <span className="label">V_oc (adjusted)</span>
          <span className="value">{ivData.vocAdj.toFixed(3)} V</span>
        </div>
      </div>

      <Plot
        data={[
          { x: ivData.vs, y: ivData.is, type: 'scatter', mode: 'lines',
            name: 'I-V curve', line: { color: '#2196F3', width: 2 }, yaxis: 'y' },
          { x: ivData.vs, y: ivData.ps, type: 'scatter', mode: 'lines',
            name: 'Power', line: { color: '#F44336', width: 2, dash: 'dash' }, yaxis: 'y2' },
          { x: [ivData.vmp], y: [ivData.imp], type: 'scatter', mode: 'markers',
            name: 'MPP', marker: { color: '#4CAF50', size: 12, symbol: 'star' } },
        ]}
        layout={{
          title: 'I-V and Power Curves',
          xaxis: { title: 'Voltage (V)' },
          yaxis: { title: 'Current (A)', side: 'left' },
          yaxis2: { title: 'Power (W)', overlaying: 'y', side: 'right' },
          margin: { t: 40, r: 60, b: 50, l: 60 }, height: 380,
          legend: { x: 0.02, y: 0.98 },
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Photovoltaic Effect</h3>
        <p><strong>Principle:</strong> Photons with energy ≥ bandgap create electron-hole pairs in the depletion region. The built-in electric field separates charges, creating current.</p>
        <p><strong>I-V equation:</strong> I = I_sc − I₀[exp(V/nV_T) − 1]</p>
        <p><strong>Fill Factor:</strong> FF = P_max / (I_sc × V_oc) — measures how "square" the I-V curve is</p>
        <p><strong>Efficiency:</strong> η = P_max / (G × A) where G = irradiance, A = cell area</p>
        <p><strong>Temperature effect:</strong> V_oc decreases ~2mV/°C, slightly increasing I_sc — net negative effect on efficiency</p>
      </div>
    </div>
  );
}
