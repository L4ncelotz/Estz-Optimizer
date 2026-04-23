import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { api, SystemOverview, LiveMetrics } from '../lib/tauri';
import { Cpu, MemoryStick, Monitor, Zap, Shield } from 'lucide-react';

export default function Dashboard() {
  const { t } = useTranslation();
  const [overview, setOverview] = useState<SystemOverview | null>(null);
  const [metrics, setMetrics] = useState<LiveMetrics | null>(null);
  const [powerPlan, setPowerPlan] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
    const interval = setInterval(loadMetrics, 3000);
    return () => clearInterval(interval);
  }, []);

  async function loadData() {
    try {
      const [sys, , plan] = await Promise.all([
        api.getSystemOverview(),
        api.getHardwareInfo(),
        api.getPowerPlan(),
      ]);
      setOverview(sys);
      setPowerPlan(plan);
      await loadMetrics();
    } catch (err) {
      console.error('Failed to load system info:', err);
    } finally {
      setLoading(false);
    }
  }

  async function loadMetrics() {
    try {
      const m = await api.getLiveMetrics();
      setMetrics(m);
    } catch (err) {
      console.error('Failed to load metrics:', err);
    }
  }

  if (loading) {
    return (
      <div className="main-content flex items-center justify-center" style={{ justifyContent: 'center' }}>
        <div className="spinner" style={{ width: 32, height: 32 }}></div>
      </div>
    );
  }

  const cpuColor = (metrics?.cpu_usage ?? 0) > 80 ? 'var(--danger)' : (metrics?.cpu_usage ?? 0) > 50 ? 'var(--warning)' : 'var(--success)';
  const ramColor = (metrics?.ram_usage ?? 0) > 80 ? 'var(--danger)' : (metrics?.ram_usage ?? 0) > 50 ? 'var(--warning)' : 'var(--accent)';

  return (
    <div className="animate-fade-in">
      <div className="page-header">
        <h2>{t('dashboard.title')}</h2>
        <p>{t('dashboard.subtitle')}</p>
      </div>

      {/* Live Gauges */}
      <div className="card-grid-3 mb-6" style={{ gridTemplateColumns: '1fr 1fr 1fr' }}>
        <div className="card" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12, padding: 24 }}>
          <div style={{ position: 'relative', width: 120, height: 120 }}>
            <svg viewBox="0 0 120 120" style={{ transform: 'rotate(-90deg)' }}>
              <circle cx="60" cy="60" r="52" fill="none" stroke="var(--bg-tertiary)" strokeWidth="8" />
              <circle cx="60" cy="60" r="52" fill="none" stroke={cpuColor} strokeWidth="8"
                strokeDasharray={`${(metrics?.cpu_usage ?? 0) * 3.267} 326.7`}
                strokeLinecap="round" style={{ transition: 'stroke-dasharray 0.5s ease' }} />
            </svg>
            <div style={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', textAlign: 'center' }}>
              <div style={{ fontSize: 28, fontWeight: 700, color: cpuColor }}>{metrics?.cpu_usage ?? 0}%</div>
            </div>
          </div>
          <div className="gauge-label">{t('dashboard.cpu_usage')}</div>
        </div>

        <div className="card" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12, padding: 24 }}>
          <div style={{ position: 'relative', width: 120, height: 120 }}>
            <svg viewBox="0 0 120 120" style={{ transform: 'rotate(-90deg)' }}>
              <circle cx="60" cy="60" r="52" fill="none" stroke="var(--bg-tertiary)" strokeWidth="8" />
              <circle cx="60" cy="60" r="52" fill="none" stroke={ramColor} strokeWidth="8"
                strokeDasharray={`${(metrics?.ram_usage ?? 0) * 3.267} 326.7`}
                strokeLinecap="round" style={{ transition: 'stroke-dasharray 0.5s ease' }} />
            </svg>
            <div style={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', textAlign: 'center' }}>
              <div style={{ fontSize: 28, fontWeight: 700, color: ramColor }}>{metrics?.ram_usage ?? 0}%</div>
            </div>
          </div>
          <div className="gauge-label">{t('dashboard.ram_usage')}</div>
          <div style={{ fontSize: 12, color: 'var(--text-tertiary)' }}>
            {metrics?.ram_used_gb ?? 0} / {metrics?.ram_total_gb ?? 0} GB
          </div>
        </div>

        <div className="card" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12, padding: 24 }}>
          <div style={{ position: 'relative', width: 120, height: 120 }}>
            <svg viewBox="0 0 120 120" style={{ transform: 'rotate(-90deg)' }}>
              <circle cx="60" cy="60" r="52" fill="none" stroke="var(--bg-tertiary)" strokeWidth="8" />
              <circle cx="60" cy="60" r="52" fill="none" stroke="#8b5cf6" strokeWidth="8"
                strokeDasharray={`${(overview?.disk_usage_percent ?? 0) * 3.267} 326.7`}
                strokeLinecap="round" style={{ transition: 'stroke-dasharray 0.5s ease' }} />
            </svg>
            <div style={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', textAlign: 'center' }}>
              <div style={{ fontSize: 28, fontWeight: 700, color: '#8b5cf6' }}>{overview?.disk_usage_percent ?? 0}%</div>
            </div>
          </div>
          <div className="gauge-label">{t('dashboard.disk')}</div>
          <div style={{ fontSize: 12, color: 'var(--text-tertiary)' }}>
            {t('dashboard.free')}: {overview?.disk_free_gb ?? 0} GB
          </div>
        </div>
      </div>

      {/* System Info Cards */}
      <div className="card-grid mb-6" style={{ gridTemplateColumns: 'repeat(2, 1fr)' }}>
        <div className="card stat-card">
          <div className="stat-icon blue"><Cpu size={22} /></div>
          <div className="stat-info">
            <h4>CPU</h4>
            <div className="stat-value" style={{ fontSize: 16 }}>{overview?.cpu_name}</div>
            <div className="stat-sub">{overview?.cpu_cores_physical} {t('dashboard.cores')} / {overview?.cpu_cores_logical} {t('dashboard.threads')}</div>
          </div>
        </div>

        <div className="card stat-card">
          <div className="stat-icon green"><MemoryStick size={22} /></div>
          <div className="stat-info">
            <h4>RAM</h4>
            <div className="stat-value" style={{ fontSize: 16 }}>{overview?.ram_total_gb} GB</div>
            <div className="stat-sub">{t('dashboard.used')}: {overview?.ram_used_gb} GB</div>
          </div>
        </div>

        <div className="card stat-card">
          <div className="stat-icon purple"><Monitor size={22} /></div>
          <div className="stat-info">
            <h4>{t('dashboard.gpu')}</h4>
            <div className="stat-value" style={{ fontSize: 16 }}>{overview?.gpu_name}</div>
          </div>
        </div>

        <div className="card stat-card">
          <div className="stat-icon amber"><Zap size={22} /></div>
          <div className="stat-info">
            <h4>{t('dashboard.power_plan')}</h4>
            <div className="stat-value" style={{ fontSize: 16 }}>{powerPlan}</div>
          </div>
        </div>
      </div>

      {/* OS Info */}
      <div className="card" style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
        <div className="stat-icon blue"><Shield size={22} /></div>
        <div>
          <div style={{ fontSize: 12, color: 'var(--text-tertiary)', fontWeight: 600, textTransform: 'uppercase' as const, letterSpacing: 0.5 }}>{t('dashboard.os')}</div>
          <div style={{ fontSize: 15, fontWeight: 600 }}>{overview?.os_name} {overview?.os_version}</div>
          <div style={{ fontSize: 13, color: 'var(--text-secondary)' }}>{overview?.hostname}</div>
        </div>
      </div>
    </div>
  );
}
