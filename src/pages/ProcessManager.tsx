import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { api, ProcessInfo, CpuTopology } from '../lib/tauri';
import { Cpu, RefreshCw } from 'lucide-react';

export default function ProcessManager() {
  const { t } = useTranslation();
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [topology, setTopology] = useState<CpuTopology | null>(null);
  const [loading, setLoading] = useState(true);
  const [actionPid, setActionPid] = useState<number | null>(null);
  const [search, setSearch] = useState('');

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    setLoading(true);
    try {
      const [procs, topo] = await Promise.all([
        api.listProcesses(),
        api.getCpuTopology(),
      ]);
      setProcesses(procs);
      setTopology(topo);
    } catch (err) {
      console.error('Failed to load processes:', err);
    } finally {
      setLoading(false);
    }
  }

  async function handlePin(pid: number, type: 'p' | 'e' | 'reset') {
    setActionPid(pid);
    try {
      if (type === 'p') await api.pinToPCores(pid);
      else if (type === 'e') await api.pinToECores(pid);
      else await api.resetAffinity(pid);
    } catch (err) {
      console.error(`Failed to set affinity for PID ${pid}:`, err);
    } finally {
      setActionPid(null);
    }
  }

  const filtered = processes.filter(p =>
    p.name.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="animate-fade-in">
      <div className="page-header flex justify-between items-center">
        <div>
          <h2>{t('process.title')}</h2>
          <p>{t('process.subtitle')}</p>
        </div>
        <button className="btn btn-ghost" onClick={loadData} disabled={loading}>
          <RefreshCw size={16} className={loading ? 'spinner' : ''} />
          {t('process.refresh')}
        </button>
      </div>

      {/* CPU Topology */}
      {topology && (
        <div className="card mb-6">
          <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 16 }}>{t('process.cpu_topology')}</h3>
          <div style={{ display: 'flex', gap: 16, flexWrap: 'wrap' }}>
            {topology.is_hybrid ? (
              <>
                <div style={{ flex: 1, minWidth: 200, padding: 16, background: 'rgba(59, 130, 246, 0.1)', borderRadius: 'var(--radius-sm)', border: '1px solid rgba(59, 130, 246, 0.2)' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
                    <Cpu size={18} style={{ color: 'var(--accent)' }} />
                    <span style={{ fontWeight: 600, color: 'var(--accent)' }}>{t('process.p_cores')}</span>
                  </div>
                  <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
                    {topology.cores.filter(c => c.core_type === 'P-Core').map(c => (
                      <div key={c.index} style={{ padding: '4px 10px', background: 'rgba(59, 130, 246, 0.2)', borderRadius: 4, fontSize: 12, fontWeight: 600 }}>
                        Core {c.index}
                      </div>
                    ))}
                  </div>
                </div>
                <div style={{ flex: 1, minWidth: 200, padding: 16, background: 'rgba(16, 185, 129, 0.1)', borderRadius: 'var(--radius-sm)', border: '1px solid rgba(16, 185, 129, 0.2)' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
                    <Cpu size={18} style={{ color: 'var(--success)' }} />
                    <span style={{ fontWeight: 600, color: 'var(--success)' }}>{t('process.e_cores')}</span>
                  </div>
                  <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
                    {topology.cores.filter(c => c.core_type === 'E-Core').map(c => (
                      <div key={c.index} style={{ padding: '4px 10px', background: 'rgba(16, 185, 129, 0.2)', borderRadius: 4, fontSize: 12, fontWeight: 600 }}>
                        Core {c.index}
                      </div>
                    ))}
                  </div>
                </div>
              </>
            ) : (
              <div style={{ padding: 16, background: 'var(--bg-tertiary)', borderRadius: 'var(--radius-sm)', width: '100%' }}>
                <p style={{ color: 'var(--text-secondary)', fontSize: 14 }}>
                  {t('process.not_hybrid')} — {topology.total_cores} {t('dashboard.cores')} / {topology.total_threads} {t('dashboard.threads')}
                </p>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Process Search */}
      <div style={{ marginBottom: 16 }}>
        <input
          type="text"
          placeholder="Search processes..."
          value={search}
          onChange={e => setSearch(e.target.value)}
          style={{
            width: '100%', maxWidth: 400, padding: '10px 16px', background: 'var(--bg-secondary)',
            border: '1px solid var(--border)', borderRadius: 'var(--radius-sm)', color: 'var(--text-primary)',
            fontFamily: 'var(--font)', fontSize: 14, outline: 'none'
          }}
        />
      </div>

      {/* Process Table */}
      <div className="card" style={{ padding: 0, overflow: 'hidden' }}>
        <div style={{ maxHeight: 500, overflowY: 'auto' }}>
          <table className="process-table">
            <thead>
              <tr>
                <th>{t('process.name')}</th>
                <th>{t('process.pid')}</th>
                <th>{t('process.cpu')}</th>
                <th>{t('process.memory')}</th>
                {topology?.is_hybrid && <th>Actions</th>}
              </tr>
            </thead>
            <tbody>
              {filtered.slice(0, 50).map(proc => (
                <tr key={proc.pid}>
                  <td style={{ fontWeight: 500, color: 'var(--text-primary)' }}>{proc.name}</td>
                  <td>{proc.pid}</td>
                  <td>
                    <span style={{ color: proc.cpu_percent > 50 ? 'var(--danger)' : proc.cpu_percent > 20 ? 'var(--warning)' : 'var(--text-secondary)' }}>
                      {proc.cpu_percent}%
                    </span>
                  </td>
                  <td>{proc.memory_mb} MB</td>
                  {topology?.is_hybrid && (
                    <td>
                      <div style={{ display: 'flex', gap: 4 }}>
                        <button className="btn btn-sm btn-primary" onClick={() => handlePin(proc.pid, 'p')} disabled={actionPid === proc.pid} style={{ padding: '3px 8px', fontSize: 11 }}>
                          P
                        </button>
                        <button className="btn btn-sm btn-success" onClick={() => handlePin(proc.pid, 'e')} disabled={actionPid === proc.pid} style={{ padding: '3px 8px', fontSize: 11, background: 'var(--success)' }}>
                          E
                        </button>
                        <button className="btn btn-sm btn-ghost" onClick={() => handlePin(proc.pid, 'reset')} disabled={actionPid === proc.pid} style={{ padding: '3px 8px', fontSize: 11 }}>
                          ↺
                        </button>
                      </div>
                    </td>
                  )}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
