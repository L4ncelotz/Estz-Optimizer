import { useTranslation } from 'react-i18next';
import { useQuery } from '@tanstack/react-query';
import { api } from '../lib/tauri';
import { Cpu, MemoryStick, Monitor, Zap, Shield } from 'lucide-react';
import { GaugeCard } from '../components/GaugeCard';
import { StatCard } from '../components/StatCard';

export default function Dashboard() {
  const { t } = useTranslation();

  const { data: overview, isLoading: loadingOverview } = useQuery({
    queryKey: ['systemOverview'],
    queryFn: api.getSystemOverview,
    staleTime: Infinity,
  });

  const { data: powerPlan, isLoading: loadingPowerPlan } = useQuery({
    queryKey: ['powerPlan'],
    queryFn: api.getPowerPlan,
    staleTime: Infinity,
  });

  const { data: metrics } = useQuery({
    queryKey: ['liveMetrics'],
    queryFn: api.getLiveMetrics,
    refetchInterval: 3000,
  });

  const loading = loadingOverview || loadingPowerPlan;

  if (loading) {
    return (
      <div className="main-content flex items-center justify-center" style={{ justifyContent: 'center' }}>
        <div className="spinner" style={{ width: 32, height: 32 }}></div>
      </div>
    );
  }

  const cpuUsage = metrics?.cpu_usage ?? 0;
  const ramUsage = metrics?.ram_usage ?? 0;

  const cpuColor = cpuUsage > 80 ? 'var(--danger)' : cpuUsage > 50 ? 'var(--warning)' : 'var(--success)';
  const ramColor = ramUsage > 80 ? 'var(--danger)' : ramUsage > 50 ? 'var(--warning)' : 'var(--accent)';

  return (
    <div className="animate-fade-in">
      <div className="page-header">
        <h2>{t('dashboard.title')}</h2>
        <p>{t('dashboard.subtitle')}</p>
      </div>

      {/* Live Gauges */}
      <div className="card-grid-3 mb-6" style={{ gridTemplateColumns: '1fr 1fr 1fr' }}>
        <GaugeCard 
          title={t('dashboard.cpu_usage')} 
          valuePercent={cpuUsage} 
          color={cpuColor} 
          centerText={`${cpuUsage}%`} 
        />
        <GaugeCard 
          title={t('dashboard.ram_usage')} 
          valuePercent={ramUsage} 
          color={ramColor} 
          centerText={`${ramUsage}%`} 
          subText={`${metrics?.ram_used_gb ?? 0} / ${metrics?.ram_total_gb ?? 0} GB`} 
        />
        <GaugeCard 
          title={t('dashboard.disk')} 
          valuePercent={overview?.disk_usage_percent ?? 0} 
          color="#8b5cf6" 
          centerText={`${overview?.disk_usage_percent ?? 0}%`} 
          subText={`${t('dashboard.free')}: ${overview?.disk_free_gb ?? 0} GB`} 
        />
      </div>

      {/* System Info Cards */}
      <div className="card-grid mb-6" style={{ gridTemplateColumns: 'repeat(2, 1fr)' }}>
        <StatCard
          colorTheme="blue" icon={<Cpu size={22} />} title="CPU"
          value={overview?.cpu_name}
          subText={`${overview?.cpu_cores_physical} ${t('dashboard.cores')} / ${overview?.cpu_cores_logical} ${t('dashboard.threads')}`}
        />
        <StatCard
          colorTheme="green" icon={<MemoryStick size={22} />} title="RAM"
          value={`${overview?.ram_total_gb} GB`}
          subText={`${t('dashboard.used')}: ${overview?.ram_used_gb} GB`}
        />
        <StatCard
          colorTheme="purple" icon={<Monitor size={22} />} title={t('dashboard.gpu')}
          value={overview?.gpu_name}
        />
        <StatCard
          colorTheme="amber" icon={<Zap size={22} />} title={t('dashboard.power_plan')}
          value={powerPlan}
        />
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
