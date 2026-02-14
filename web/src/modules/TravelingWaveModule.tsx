import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function TravelingWaveModule() {
  const [amplitude, setAmplitude] = useState(1.0);
  const [frequency, setFrequency] = useState(1.0);
  const [time, setTime] = useState(0);

  const xMax = 4 / frequency;

  const data = useMemo(
    () => wasm.traveling_wave_snapshot(amplitude, frequency, time, xMax, 500),
    [amplitude, frequency, time, xMax],
  );

  const period = 1 / frequency;

  return (
    <div className="module-panel">
      <h2>Traveling Wave Snapshot</h2>
      <div className="controls">
        <div className="control-group">
          <label>Amplitude</label>
          <input type="number" value={amplitude} onChange={e => setAmplitude(+e.target.value)} step={0.1} min={0} />
        </div>
        <div className="control-group">
          <label>Frequency (Hz)</label>
          <input type="number" value={frequency} onChange={e => setFrequency(+e.target.value)} step={0.5} min={0.1} />
        </div>
        <div className="control-group">
          <label>Time (s)</label>
          <input type="range" min={0} max={period * 2} step={period / 100} value={time}
            onChange={e => setTime(+e.target.value)} />
          <span>{time.toFixed(3)}s</span>
        </div>
      </div>
      <Plot
        data={[{
          x: data.positions, y: data.values, mode: 'lines',
          line: { color: '#e63946', width: 2 }, name: 'E(x,t)',
        }]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Position (m)' } },
          yaxis: { title: { text: 'Amplitude' }, range: [-amplitude * 1.2, amplitude * 1.2] },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
    </div>
  );
}
