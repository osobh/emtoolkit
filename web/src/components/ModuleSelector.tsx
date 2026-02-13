import './ModuleSelector.css';

interface Module {
  id: string;
  name: string;
  chapter: number;
}

interface Props {
  modules: Module[];
  active: string;
  onSelect: (id: string) => void;
}

export function ModuleSelector({ modules, active, onSelect }: Props) {
  const chapters = [...new Set(modules.map(m => m.chapter))].sort();

  return (
    <nav className="module-selector">
      {chapters.map(ch => (
        <div key={ch} className="chapter-group">
          <div className="chapter-label">Chapter {ch}</div>
          {modules.filter(m => m.chapter === ch).map(m => (
            <button
              key={m.id}
              className={`module-btn ${m.id === active ? 'active' : ''}`}
              onClick={() => onSelect(m.id)}
            >
              {m.name}
            </button>
          ))}
        </div>
      ))}
    </nav>
  );
}
