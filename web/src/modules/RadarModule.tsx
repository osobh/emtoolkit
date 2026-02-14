import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function RadarModule() {
  const [ptx, setPtx] = useState(1000);
  const [gtx, setGtx] = useState(30);
  const [grx, setGrx] = useState(30);
  const [frequency, setFrequency] = useState(10e9);
  const [rcs, setRcs] = useState(1.0);
  const [rMin, setRMin] = useState(1000);
  const [rMax, setRMax] = useState(100000);
  const [noiseFigure, setNoiseFigure] = useState(3);
  const [bandwidth, setBandwidth] = useState(1e6);
  const [losses, setLosses] = useState(6);
  const [snrReq, setSnrReq] = useState(13);

  const lambda = 3e8 / frequency;

  const results = useMemo(() => {
    const gTxLin = 10 ** (gtx / 10);
    const gRxLin = 10 ** (grx / 10);
    const lossLin = 10 ** (losses / 10);
    const nfLin = 10 ** (noiseFigure / 10);
    const kB = 1.38e-23;
    const t0 = 290;
    const noiseFloor = kB * t0 * bandwidth * nfLin;
    const noiseFloorDbm = 10 * Math.log10(noiseFloor) + 30;

    // Radar equation: Pr = Pt Gt Gr λ² σ / ((4π)³ R⁴ L)
    const numerator = ptx * gTxLin * gRxLin * lambda * lambda * rcs;
    const denominator = Math.pow(4 * Math.PI, 3) * lossLin;

    // Max range for given SNR
    const snrLin = 10 ** (snrReq / 10);
    const rMaxCalc = Math.pow(numerator / (denominator * snrLin * noiseFloor), 0.25);

    // Range profile
    const n = 300;
    const ranges: number[] = [];
    const prDbm: number[] = [];
    const snrDb: number[] = [];
    for (let i = 0; i < n; i++) {
      const r = rMin + (rMax - rMin) * i / (n - 1);
      ranges.push(r / 1000); // km
      const pr = numerator / (denominator * r ** 4);
      prDbm.push(10 * Math.log10(pr) + 30);
      snrDb.push(10 * Math.log10(pr / noiseFloor));
    }

    return { rMaxCalc, noiseFloorDbm, ranges, prDbm, snrDb };
  }, [ptx, gtx, grx, frequency, rcs, rMin, rMax, noiseFigure, bandwidth, losses, snrReq, lambda]);

  return (
    <div className="module">
      <h2>Radar Range Equation</h2>
      <p>Calculate received power, SNR, and maximum detection range for a monostatic radar.</p>

      <div className="controls">
        <label>P_tx (W): <input type="range" min={1} max={100000} step={100} value={ptx}
          onChange={e => setPtx(+e.target.value)} />
          {ptx >= 1000 ? (ptx / 1000).toFixed(1) + ' kW' : ptx + ' W'}</label>
        <label>G_tx (dBi): <input type="range" min={0} max={50} step={0.5} value={gtx}
          onChange={e => setGtx(+e.target.value)} /> {gtx}</label>
        <label>G_rx (dBi): <input type="range" min={0} max={50} step={0.5} value={grx}
          onChange={e => setGrx(+e.target.value)} /> {grx}</label>
        <label>Frequency: <input type="range" min={1e8} max={100e9} step={1e8} value={frequency}
          onChange={e => setFrequency(+e.target.value)} />
          {frequency >= 1e9 ? (frequency / 1e9).toFixed(1) + ' GHz' : (frequency / 1e6).toFixed(0) + ' MHz'}</label>
        <label>RCS σ (m²): <input type="range" min={0.01} max={100} step={0.01} value={rcs}
          onChange={e => setRcs(+e.target.value)} /> {rcs}</label>
        <label>Noise figure (dB): <input type="range" min={1} max={15} step={0.5} value={noiseFigure}
          onChange={e => setNoiseFigure(+e.target.value)} /> {noiseFigure}</label>
        <label>Bandwidth: <input type="range" min={3} max={9} step={0.1}
          value={Math.log10(bandwidth)} onChange={e => setBandwidth(10 ** +e.target.value)} />
          {bandwidth >= 1e6 ? (bandwidth / 1e6).toFixed(1) + ' MHz' : (bandwidth / 1e3).toFixed(0) + ' kHz'}</label>
        <label>System losses (dB): <input type="range" min={0} max={20} step={0.5} value={losses}
          onChange={e => setLosses(+e.target.value)} /> {losses}</label>
        <label>Required SNR (dB): <input type="range" min={5} max={25} step={1} value={snrReq}
          onChange={e => setSnrReq(+e.target.value)} /> {snrReq}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Max detection range</span>
          <span className="value">{(results.rMaxCalc / 1000).toFixed(1)} km</span>
        </div>
        <div className="result-card">
          <span className="label">Noise floor</span>
          <span className="value">{results.noiseFloorDbm.toFixed(1)} dBm</span>
        </div>
        <div className="result-card">
          <span className="label">Wavelength</span>
          <span className="value">{(lambda * 100).toFixed(2)} cm</span>
        </div>
      </div>

      <Plot
        data={[
          { x: results.ranges, y: results.snrDb, type: 'scatter', mode: 'lines',
            name: 'SNR (dB)', line: { color: '#2196F3', width: 2 } },
          { x: [results.ranges[0], results.ranges[results.ranges.length - 1]],
            y: [snrReq, snrReq], type: 'scatter', mode: 'lines',
            name: `Required SNR (${snrReq} dB)`,
            line: { color: '#F44336', width: 1, dash: 'dash' } },
        ]}
        layout={{
          title: 'SNR vs Range',
          xaxis: { title: 'Range (km)' },
          yaxis: { title: 'SNR (dB)' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 380,
          legend: { x: 0.6, y: 0.95 },
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Radar Range Equation</h3>
        <p><strong>P_r = P_t G_t G_r λ² σ / ((4π)³ R⁴ L)</strong></p>
        <p><strong>R_max = [P_t G_t G_r λ² σ / ((4π)³ L · SNR_min · kTBF)]^(1/4)</strong></p>
        <p>Key: Power falls as R⁴ (round trip), so doubling range requires 16× more power.</p>
      </div>
    </div>
  );
}
