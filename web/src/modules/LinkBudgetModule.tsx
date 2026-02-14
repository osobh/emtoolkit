import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function LinkBudgetModule() {
  const [pTxW, setPTxW] = useState(1.0);
  const [gTxDb, setGTxDb] = useState(10);
  const [gRxDb, setGRxDb] = useState(10);
  const [frequency, setFrequency] = useState(2.4e9);
  const [distance, setDistance] = useState(1000);

  const point = useMemo(
    () => wasm.friis_link(pTxW, gTxDb, gRxDb, frequency, distance),
    [pTxW, gTxDb, gRxDb, frequency, distance],
  );

  const curve = useMemo(
    () => wasm.link_vs_distance(pTxW, gTxDb, gRxDb, frequency, 1, distance * 3, 300),
    [pTxW, gTxDb, gRxDb, frequency, distance],
  );

  return (
    <div className="module-panel">
      <h2>Friis Link Budget</h2>
      <div className="controls">
        <div className="control-group"><label>P_tx (W)</label><input type="number" value={pTxW} onChange={e => setPTxW(+e.target.value)} step={0.5} min={0.001} /></div>
        <div className="control-group"><label>G_tx (dBi)</label><input type="number" value={gTxDb} onChange={e => setGTxDb(+e.target.value)} step={1} /></div>
        <div className="control-group"><label>G_rx (dBi)</label><input type="number" value={gRxDb} onChange={e => setGRxDb(+e.target.value)} step={1} /></div>
        <div className="control-group"><label>Freq (GHz)</label><input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.001} /></div>
        <div className="control-group"><label>Distance (m)</label><input type="number" value={distance} onChange={e => setDistance(+e.target.value)} step={100} min={1} /></div>
      </div>

      <Plot
        data={[
          {
            x: curve.distances, y: curve.p_rx_dbm, mode: 'lines',
            line: { color: '#2196f3', width: 2 }, name: 'P_rx (dBm)',
          },
          {
            x: [distance], y: [point.p_rx_dbm], mode: 'markers',
            marker: { size: 12, color: '#e63946' }, name: `${point.p_rx_dbm.toFixed(1)} dBm @ ${distance}m`,
          },
        ]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Distance (m)' }, type: 'log' },
          yaxis: { title: { text: 'Received Power (dBm)' } },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          legend: { x: 0.6, y: 0.98 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />

      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">P_rx</span><span className="result-value">{point.p_rx_dbm.toFixed(2)} dBm</span></div>
          <div className="result-item"><span className="result-label">P_rx</span><span className="result-value">{point.p_rx_w.toExponential(3)} W</span></div>
          <div className="result-item"><span className="result-label">Path Loss</span><span className="result-value">{point.path_loss_db.toFixed(2)} dB</span></div>
          <div className="result-item"><span className="result-label">FSPL</span><span className="result-value">{point.fspl_db.toFixed(2)} dB</span></div>
        </div>
      </div>
    </div>
  );
}
