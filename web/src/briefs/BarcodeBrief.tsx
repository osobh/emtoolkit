import { useState, useMemo } from 'react';

export function BarcodeBrief() {
  const [digits, setDigits] = useState('5901234123457');
  const [scanPos, setScanPos] = useState(50);

  // EAN-13 encoding (simplified visual)
  const bars = useMemo(() => {
    const result: { x: number; w: number; black: boolean }[] = [];
    let x = 0;
    const moduleWidth = 2;
    // Start guard
    for (const b of [true, false, true]) { result.push({ x, w: moduleWidth, black: b }); x += moduleWidth; }
    // Data bars (simplified — alternating for visual)
    for (let i = 0; i < 42; i++) {
      const black = i % 2 === 0;
      const w = moduleWidth * (1 + Math.floor(Math.random() * 3));
      result.push({ x, w, black }); x += w;
    }
    // Center guard
    for (const b of [false, true, false, true, false]) { result.push({ x, w: moduleWidth, black: b }); x += moduleWidth; }
    // More data
    for (let i = 0; i < 42; i++) {
      const black = i % 2 === 0;
      const w = moduleWidth * (1 + Math.floor(Math.random() * 3));
      result.push({ x, w, black }); x += w;
    }
    // End guard
    for (const b of [true, false, true]) { result.push({ x, w: moduleWidth, black: b }); x += moduleWidth; }
    return { bars: result, totalWidth: x };
  }, []);

  return (
    <div className="module">
      <h2>TB16: Bar-Code Readers</h2>
      <p>Barcode scanners use focused light (laser or LED) and photodetectors to read patterns of reflected/absorbed light encoding data.</p>

      <div className="controls">
        <label>UPC/EAN digits: <input type="text" value={digits} onChange={e => setDigits(e.target.value)}
          style={{ width: 160 }} maxLength={13} /></label>
        <label>Scan position: <input type="range" min={0} max={100} step={1} value={scanPos}
          onChange={e => setScanPos(+e.target.value)} /></label>
      </div>

      {/* Visual barcode */}
      <div style={{ margin: '16px 0', padding: '10px 20px', background: 'white', borderRadius: 8, border: '1px solid #ddd', position: 'relative' }}>
        <svg width="100%" height={80} viewBox={`0 0 ${bars.totalWidth} 60`} preserveAspectRatio="none">
          {bars.bars.map((b, i) =>
            b.black ? <rect key={i} x={b.x} y={0} width={b.w} height={60} fill="black" /> : null
          )}
          {/* Scan line */}
          <line x1={bars.totalWidth * scanPos / 100} y1={0} x2={bars.totalWidth * scanPos / 100} y2={60}
            stroke="red" strokeWidth={1} opacity={0.8} />
        </svg>
        <div style={{ textAlign: 'center', fontFamily: 'monospace', fontSize: 16, letterSpacing: 4 }}>{digits}</div>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Encoding</span><span className="value">EAN-13 / UPC-A</span></div>
        <div className="result-card"><span className="label">Light source</span><span className="value">650 nm laser diode (red)</span></div>
        <div className="result-card"><span className="label">Module width</span><span className="value">0.264 mm (standard)</span></div>
        <div className="result-card"><span className="label">Data capacity</span><span className="value">13 digits (EAN-13)</span></div>
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: '#E3F2FD', borderRadius: 8 }}>
        <h4>How Scanning Works</h4>
        <p style={{ fontSize: 14 }}><strong>1.</strong> Laser/LED illuminates barcode with focused beam.</p>
        <p style={{ fontSize: 14 }}><strong>2.</strong> Black bars absorb light; white spaces reflect it back.</p>
        <p style={{ fontSize: 14 }}><strong>3.</strong> Photodetector converts reflected light intensity to electrical signal.</p>
        <p style={{ fontSize: 14 }}><strong>4.</strong> Signal processor identifies bar/space widths (1-4 modules each).</p>
        <p style={{ fontSize: 14 }}><strong>5.</strong> Decoder matches patterns to digits using encoding tables.</p>
      </div>

      <div className="theory">
        <h3>EM Concepts</h3>
        <p><strong>Reflection vs absorption:</strong> White spaces reflect ~90% of incident light; black bars absorb ~95%. Contrast ratio is critical.</p>
        <p><strong>Laser advantage:</strong> Coherent, monochromatic beam allows longer read distances and better signal-to-noise than LED illumination.</p>
        <p><strong>2D codes (QR):</strong> Encode data in both dimensions. Use camera-based imaging instead of single-line scanning. Store 100× more data.</p>
      </div>
    </div>
  );
}
