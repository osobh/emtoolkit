import { useState, useMemo } from 'react';

const LED_TYPES = [
  { name: 'Incandescent', efficacy: 15, cct: 2700, wavelength: null, lifetime: 1000, color: '#FFD700' },
  { name: 'CFL', efficacy: 60, cct: 4100, wavelength: null, lifetime: 8000, color: '#FFF5E6' },
  { name: 'White LED', efficacy: 150, cct: 5000, wavelength: null, lifetime: 50000, color: '#FFFFF0' },
  { name: 'Red LED', efficacy: 45, cct: null, wavelength: 625, lifetime: 50000, color: '#FF0000' },
  { name: 'Green LED', efficacy: 100, cct: null, wavelength: 525, lifetime: 50000, color: '#00FF00' },
  { name: 'Blue LED', efficacy: 30, cct: null, wavelength: 470, lifetime: 50000, color: '#0066FF' },
  { name: 'Warm White LED', efficacy: 120, cct: 2700, wavelength: null, lifetime: 50000, color: '#FFE4B5' },
  { name: 'Cool White LED', efficacy: 160, cct: 6500, wavelength: null, lifetime: 50000, color: '#F0F8FF' },
];

export function LEDBrief() {
  const [power, setPower] = useState(10);
  const [selected, setSelected] = useState(2);
  const [cct, setCct] = useState(5000);

  const led = LED_TYPES[selected];
  const lumens = power * led.efficacy;
  const heatWaste = power * (1 - led.efficacy / 683); // 683 lm/W is theoretical max
  const yearlyKwh = power * 8 * 365 / 1000; // 8h/day
  const yearlyCost = yearlyKwh * 0.12;
  const yearsLife = led.lifetime / (8 * 365);

  // CCT to approximate RGB
  const cctColor = useMemo(() => {
    const t = cct / 100;
    let r, g, b;
    if (t <= 66) {
      r = 255;
      g = Math.min(255, Math.max(0, 99.4708 * Math.log(t) - 161.1196));
      b = t <= 19 ? 0 : Math.min(255, Math.max(0, 138.5177 * Math.log(t - 10) - 305.0448));
    } else {
      r = Math.min(255, Math.max(0, 329.6987 * Math.pow(t - 60, -0.1332)));
      g = Math.min(255, Math.max(0, 288.1222 * Math.pow(t - 60, -0.0755)));
      b = 255;
    }
    return `rgb(${Math.round(r)},${Math.round(g)},${Math.round(b)})`;
  }, [cct]);

  return (
    <div className="module">
      <h2>TB1: LED Lighting</h2>
      <p>Light-emitting diodes convert electrical energy directly to photons through electroluminescence in semiconductor p-n junctions.</p>

      <div className="controls">
        <label>Light source:
          <select value={selected} onChange={e => setSelected(+e.target.value)}>
            {LED_TYPES.map((l, i) => <option key={i} value={i}>{l.name}</option>)}
          </select>
        </label>
        <label>Power (W): <input type="range" min={1} max={100} step={1} value={power}
          onChange={e => setPower(+e.target.value)} /> {power}</label>
        <label>Color temperature (K): <input type="range" min={1800} max={10000} step={100} value={cct}
          onChange={e => setCct(+e.target.value)} /> {cct}K</label>
      </div>

      <div style={{ display: 'flex', gap: 16, margin: '16px 0' }}>
        <div style={{ width: 80, height: 80, borderRadius: '50%', background: led.color,
          boxShadow: `0 0 30px ${led.color}`, border: '2px solid #ddd' }} />
        <div style={{ width: 80, height: 80, borderRadius: '50%', background: cctColor,
          boxShadow: `0 0 30px ${cctColor}`, border: '2px solid #ddd' }} />
        <div style={{ fontSize: 12, color: '#666', alignSelf: 'center' }}>
          Left: source type • Right: CCT slider preview
        </div>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Luminous output</span>
          <span className="value">{lumens.toFixed(0)} lm</span>
        </div>
        <div className="result-card">
          <span className="label">Luminous efficacy</span>
          <span className="value">{led.efficacy} lm/W</span>
        </div>
        <div className="result-card">
          <span className="label">Heat waste</span>
          <span className="value">{heatWaste.toFixed(1)} W</span>
        </div>
        <div className="result-card">
          <span className="label">Lifetime</span>
          <span className="value">{(led.lifetime / 1000).toFixed(0)}k hrs ({yearsLife.toFixed(1)} yrs @ 8h/day)</span>
        </div>
        <div className="result-card">
          <span className="label">Annual energy (8h/day)</span>
          <span className="value">{yearlyKwh.toFixed(1)} kWh (${yearlyCost.toFixed(2)})</span>
        </div>
        {led.wavelength && <div className="result-card">
          <span className="label">Peak wavelength</span>
          <span className="value">{led.wavelength} nm</span>
        </div>}
      </div>

      <h3>Efficacy Comparison</h3>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
        {LED_TYPES.map((l, i) => (
          <div key={i} style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <span style={{ width: 120, fontSize: 13 }}>{l.name}</span>
            <div style={{ flex: 1, background: '#eee', borderRadius: 4, height: 20, position: 'relative' }}>
              <div style={{ width: `${l.efficacy / 683 * 100}%`, background: l.color === '#FFFFF0' ? '#2196F3' : l.color,
                height: '100%', borderRadius: 4, minWidth: 2, opacity: 0.8 }} />
            </div>
            <span style={{ width: 60, fontSize: 12, textAlign: 'right' }}>{l.efficacy} lm/W</span>
          </div>
        ))}
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <span style={{ width: 120, fontSize: 13, color: '#999' }}>Theoretical max</span>
          <div style={{ flex: 1, borderTop: '1px dashed #999' }} />
          <span style={{ width: 60, fontSize: 12, textAlign: 'right', color: '#999' }}>683 lm/W</span>
        </div>
      </div>

      <div className="theory">
        <h3>How LEDs Work</h3>
        <p>When current flows through a forward-biased p-n junction in a direct-bandgap semiconductor, electrons recombine with holes and release energy as <strong>photons</strong>. The photon wavelength depends on the bandgap energy: λ = hc/E_g.</p>
        <p><strong>White LEDs</strong> use a blue LED (GaN/InGaN) coated with a yellow phosphor (YAG:Ce) to produce broad-spectrum white light.</p>
        <p><strong>Efficacy</strong> measures lumens per watt. The theoretical maximum is 683 lm/W (monochromatic 555nm green, matching peak human eye sensitivity).</p>
      </div>
    </div>
  );
}
