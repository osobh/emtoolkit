import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function WaveformModule() {
  const [amplitude, setAmplitude] = useState(1.0);
  const [frequency, setFrequency] = useState(1.0);
  const [phase, setPhase] = useState(0);
  const [damping, setDamping] = useState(0);

  const data = useMemo(() => {
    const periods = 4;
    const tEnd = periods / frequency;
    return wasm.sinusoidal_wave(amplitude, frequency, phase, damping, tEnd, 500);
  }, [amplitude, frequency, phase, damping]);

  return (
    <div className="module-panel">
      <h2>Sinusoidal Waveforms</h2>
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
          <label>Phase (rad)</label>
          <input type="range" min={0} max={6.28} step={0.1} value={phase}
            onChange={e => setPhase(+e.target.value)} />
          <span>{phase.toFixed(1)}</span>
        </div>
        <div className="control-group">
          <label>Damping (Np/s)</label>
          <input type="range" min={0} max={5} step={0.1} value={damping}
            onChange={e => setDamping(+e.target.value)} />
          <span>{damping.toFixed(1)}</span>
        </div>
      </div>

      <Plot
        data={[{
          x: data.times,
          y: data.values,
          mode: 'lines',
          line: { color: '#2196f3', width: 2 },
          name: 'v(t)',
        }]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Time (s)' } },
          yaxis: { title: { text: 'Amplitude' } },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
    </div>
  );
}
