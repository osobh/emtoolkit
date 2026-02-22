import { Component, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  moduleName?: string;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ModuleErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="module-error" style={{
          padding: 24,
          background: '#fff5f5',
          border: '1px solid #feb2b2',
          borderRadius: 8,
          margin: 16,
        }}>
          <h3 style={{ color: '#c53030', marginTop: 0 }}>
            ⚠️ {this.props.moduleName || 'Module'} failed to render
          </h3>
          <p style={{ color: '#742a2a', fontSize: '0.9rem' }}>
            {this.state.error?.message || 'Unknown error'}
          </p>
          <button
            onClick={() => this.setState({ hasError: false, error: null })}
            style={{
              padding: '8px 16px',
              background: '#e53e3e',
              color: 'white',
              border: 'none',
              borderRadius: 4,
              cursor: 'pointer',
            }}
          >
            Retry
          </button>
        </div>
      );
    }
    return this.props.children;
  }
}
