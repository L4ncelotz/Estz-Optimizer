import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { api, OptimizationItem } from '../lib/tauri';
import { RotateCcw, AlertTriangle, ChevronDown, ChevronUp } from 'lucide-react';

const CATEGORIES = ['all', 'privacy', 'performance', 'gaming', 'visual', 'network', 'cleanup', 'boot', 'system'];

export default function AdvancedTweaks() {
  const { t, i18n } = useTranslation();
  const [catalog, setCatalog] = useState<OptimizationItem[]>([]);
  const [activeTab, setActiveTab] = useState('all');
  const [loading, setLoading] = useState(true);
  const [togglingId, setTogglingId] = useState<string | null>(null);
  const [expandedId, setExpandedId] = useState<string | null>(null);

  useEffect(() => {
    loadCatalog();
  }, []);

  async function loadCatalog() {
    try {
      const items = await api.getOptimizationCatalog();
      setCatalog(items);
    } catch (err) {
      console.error('Failed to load catalog:', err);
    } finally {
      setLoading(false);
    }
  }

  async function handleToggle(item: OptimizationItem) {
    setTogglingId(item.id);
    try {
      if (item.is_enabled) {
        await api.revertOptimization(item.id);
      } else {
        await api.applyOptimization(item.id);
      }
      // Refresh catalog
      const items = await api.getOptimizationCatalog();
      setCatalog(items);
    } catch (err) {
      console.error(`Failed to toggle ${item.id}:`, err);
    } finally {
      setTogglingId(null);
    }
  }

  const filtered = activeTab === 'all' ? catalog : catalog.filter(c => c.category === activeTab);
  const isTh = i18n.language === 'th';

  const tabLabels: Record<string, string> = {
    all: t('advanced.tab_all'),
    privacy: t('advanced.tab_privacy'),
    performance: t('advanced.tab_performance'),
    gaming: t('advanced.tab_gaming'),
    visual: t('advanced.tab_visual'),
    network: t('advanced.tab_network'),
    cleanup: t('advanced.tab_cleanup'),
    boot: t('advanced.tab_boot'),
    system: t('advanced.tab_system'),
  };

  const riskLabel = (level: string) => {
    switch (level) {
      case 'none': return t('risk.none');
      case 'low': return t('risk.low');
      case 'medium': return t('risk.medium');
      case 'high': return t('risk.high');
      default: return level;
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center" style={{ height: '100%', justifyContent: 'center' }}>
        <div className="spinner" style={{ width: 32, height: 32 }}></div>
      </div>
    );
  }

  return (
    <div className="animate-fade-in">
      <div className="page-header flex justify-between items-center">
        <div>
          <h2>{t('advanced.title')}</h2>
          <p>{t('advanced.subtitle')}</p>
        </div>
      </div>

      {/* Category Tabs */}
      <div className="tabs" style={{ flexWrap: 'wrap' }}>
        {CATEGORIES.map(cat => (
          <button
            key={cat}
            className={`tab ${activeTab === cat ? 'active' : ''}`}
            onClick={() => setActiveTab(cat)}
          >
            {tabLabels[cat] || cat}
          </button>
        ))}
      </div>

      {/* Toggle Cards */}
      <div className="flex-col gap-3" style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
        {filtered.map(item => {
          const isExpanded = expandedId === item.id;
          return (
            <div key={item.id} className="toggle-card" style={{ flexDirection: 'column', gap: 0 }}>
              <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', width: '100%', gap: 16 }}>
                <div className="toggle-card-info" style={{ flex: 1 }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4 }}>
                    <h4>{isTh ? item.name_th : item.name}</h4>
                    <span className={`badge badge-${item.risk_level}`}>{riskLabel(item.risk_level)}</span>
                    {item.requires_reboot && (
                      <span className="badge badge-medium" style={{ fontSize: 10 }}>
                        <RotateCcw size={10} /> {t('advanced.reboot_required')}
                      </span>
                    )}
                  </div>
                  <p>{isTh ? item.description_th : item.description}</p>
                </div>

                <div className="toggle-card-actions">
                  <button
                    className="btn btn-sm btn-ghost"
                    onClick={() => setExpandedId(isExpanded ? null : item.id)}
                    style={{ padding: '4px 8px' }}
                  >
                    {isExpanded ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
                  </button>

                  <label className="toggle">
                    <input
                      type="checkbox"
                      checked={item.is_enabled}
                      onChange={() => handleToggle(item)}
                      disabled={togglingId === item.id}
                    />
                    <span className="toggle-slider"></span>
                  </label>
                </div>
              </div>

              {/* Expanded pros/cons */}
              {isExpanded && (
                <div style={{ marginTop: 12, paddingTop: 12, borderTop: '1px solid var(--border)', display: 'flex', gap: 24 }}>
                  <div style={{ flex: 1 }}>
                    <div style={{ fontSize: 12, fontWeight: 600, color: 'var(--success)', marginBottom: 6, textTransform: 'uppercase' as const }}>{t('advanced.pros')}</div>
                    {item.pros.map((p, i) => (
                      <div key={i} style={{ fontSize: 13, color: 'var(--text-secondary)', marginBottom: 4 }}>✓ {p}</div>
                    ))}
                  </div>
                  {item.cons.length > 0 && (
                    <div style={{ flex: 1 }}>
                      <div style={{ fontSize: 12, fontWeight: 600, color: 'var(--warning)', marginBottom: 6, textTransform: 'uppercase' as const }}>{t('advanced.cons')}</div>
                      {item.cons.map((c, i) => (
                        <div key={i} style={{ fontSize: 13, color: 'var(--text-secondary)', marginBottom: 4 }}>
                          <AlertTriangle size={12} style={{ display: 'inline', marginRight: 4, color: 'var(--warning)' }} /> {c}
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
