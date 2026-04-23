import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { api, ActionRecord } from '../lib/tauri';
import { Clock, RotateCcw, Trash2, Download } from 'lucide-react';

export default function ActionHistory() {
  const { t } = useTranslation();
  const [history, setHistory] = useState<ActionRecord[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadHistory();
  }, []);

  async function loadHistory() {
    try {
      const records = await api.getActionHistory();
      setHistory(records);
    } catch (err) {
      console.error('Failed to load history:', err);
    } finally {
      setLoading(false);
    }
  }

  async function handleUndo(record: ActionRecord) {
    try {
      await api.revertOptimization(record.action_id);
      await loadHistory();
    } catch (err) {
      console.error('Failed to undo:', err);
    }
  }

  async function handleClear() {
    if (confirm('Clear all action history?')) {
      try {
        await api.clearActionHistory();
        setHistory([]);
      } catch (err) {
        console.error('Failed to clear:', err);
      }
    }
  }

  function handleExport() {
    const data = JSON.stringify(history, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `estz-history-${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  const statusColors: Record<string, string> = {
    Applied: 'var(--success)',
    Reverted: 'var(--warning)',
    Failed: 'var(--danger)',
  };

  const statusLabels: Record<string, string> = {
    Applied: t('history.applied'),
    Reverted: t('history.reverted'),
    Failed: t('history.failed'),
  };

  const categoryEmojis: Record<string, string> = {
    privacy: '🛡️',
    performance: '⚡',
    gaming: '🎮',
    visual: '🎨',
    network: '🌐',
    cleanup: '🧹',
    boot: '🔧',
    system: '💻',
  };

  return (
    <div className="animate-fade-in">
      <div className="page-header flex justify-between items-center">
        <div>
          <h2>{t('history.title')}</h2>
          <p>{t('history.subtitle')}</p>
        </div>
        <div style={{ display: 'flex', gap: 8 }}>
          {history.length > 0 && (
            <>
              <button className="btn btn-ghost btn-sm" onClick={handleExport}>
                <Download size={14} /> {t('history.export')}
              </button>
              <button className="btn btn-danger btn-sm" onClick={handleClear}>
                <Trash2 size={14} /> {t('history.clear')}
              </button>
            </>
          )}
        </div>
      </div>

      {loading ? (
        <div className="card" style={{ padding: 48, textAlign: 'center' }}>
          <div className="spinner" style={{ width: 32, height: 32, margin: '0 auto' }}></div>
        </div>
      ) : history.length === 0 ? (
        <div className="card" style={{ padding: 48, textAlign: 'center' }}>
          <Clock size={48} style={{ color: 'var(--text-tertiary)', margin: '0 auto 16px' }} />
          <p style={{ color: 'var(--text-secondary)' }}>{t('history.no_history')}</p>
        </div>
      ) : (
        <div className="card" style={{ padding: 8 }}>
          <div className="timeline">
            {history.map(record => (
              <div key={record.id} className="timeline-item">
                <div className={`timeline-dot ${record.status.toLowerCase()}`}></div>
                <div className="timeline-content" style={{ flex: 1 }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                    <div>
                      <h4>
                        {categoryEmojis[record.category] || '📝'} {record.action_name}
                      </h4>
                      <p>
                        <span className="badge" style={{
                          background: `${statusColors[record.status]}22`,
                          color: statusColors[record.status],
                          fontSize: 10,
                          padding: '2px 8px',
                        }}>
                          {statusLabels[record.status] || record.status}
                        </span>
                        {record.previous_value && (
                          <span style={{ marginLeft: 8, fontSize: 12, color: 'var(--text-tertiary)' }}>
                            {record.previous_value} → {record.new_value}
                          </span>
                        )}
                      </p>
                      <div className="timeline-time">
                        {new Date(record.timestamp).toLocaleString()}
                      </div>
                    </div>
                    {!record.reverted && record.status === 'Applied' && record.previous_value && (
                      <button className="btn btn-sm btn-ghost" onClick={() => handleUndo(record)}>
                        <RotateCcw size={12} /> {t('history.undo')}
                      </button>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
