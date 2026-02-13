import { useState, useEffect } from 'react';
import { ensureInit } from './wasm';
import { ModuleSelector } from './components/ModuleSelector';
import { SmithChartModule } from './modules/SmithChartModule';
import { PolarizationModule } from './modules/PolarizationModule';
import { DipoleModule } from './modules/DipoleModule';
import { FresnelModule } from './modules/FresnelModule';
import { WaveformModule } from './modules/WaveformModule';
import './App.css';

const MODULES = [
  { id: 'waveform', name: '1.1 Sinusoidal Waveforms', chapter: 1 },
  { id: 'smith', name: '2.1 Smith Chart', chapter: 2 },
  { id: 'polarization', name: '7.3 Polarization', chapter: 7 },
  { id: 'fresnel', name: '7.4 Fresnel Coefficients', chapter: 7 },
  { id: 'dipole', name: '8.1 Dipole Antennas', chapter: 8 },
];

function App() {
  const [ready, setReady] = useState(false);
  const [activeModule, setActiveModule] = useState('smith');

  useEffect(() => {
    ensureInit().then(() => setReady(true));
  }, []);

  if (!ready) {
    return <div className="loading">Loading EM Toolkit WASM engine...</div>;
  }

  const renderModule = () => {
    switch (activeModule) {
      case 'waveform': return <WaveformModule />;
      case 'smith': return <SmithChartModule />;
      case 'polarization': return <PolarizationModule />;
      case 'fresnel': return <FresnelModule />;
      case 'dipole': return <DipoleModule />;
      default: return <div>Select a module</div>;
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>⚡ EM Toolkit</h1>
        <span className="subtitle">Interactive Electromagnetics Education Platform</span>
      </header>
      <div className="app-body">
        <ModuleSelector
          modules={MODULES}
          active={activeModule}
          onSelect={setActiveModule}
        />
        <main className="module-content">
          {renderModule()}
        </main>
      </div>
    </div>
  );
}

export default App;
