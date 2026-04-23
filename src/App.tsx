import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import './i18n';
import './styles/globals.css';

import Dashboard from './pages/Dashboard';
import SafeOptimize from './pages/SafeOptimize';
import AdvancedTweaks from './pages/AdvancedTweaks';
import ProcessManager from './pages/ProcessManager';
import BloatwareRemover from './pages/BloatwareRemover';
import ActionHistory from './pages/ActionHistory';
import Settings from './pages/Settings';

import {
  LayoutDashboard,
  Zap,
  SlidersHorizontal,
  Cpu,
  Trash2,
  History,
  Settings as SettingsIcon,
} from 'lucide-react';

type Page = 'dashboard' | 'safe_optimize' | 'advanced' | 'process' | 'bloatware' | 'history' | 'settings';

interface NavItem {
  id: Page;
  labelKey: string;
  icon: React.ReactNode;
  section: 'main' | 'tools' | 'system';
}

const NAV_ITEMS: NavItem[] = [
  { id: 'dashboard', labelKey: 'nav.dashboard', icon: <LayoutDashboard size={18} />, section: 'main' },
  { id: 'safe_optimize', labelKey: 'nav.safe_optimize', icon: <Zap size={18} />, section: 'main' },
  { id: 'advanced', labelKey: 'nav.advanced', icon: <SlidersHorizontal size={18} />, section: 'main' },
  { id: 'process', labelKey: 'nav.process', icon: <Cpu size={18} />, section: 'tools' },
  { id: 'bloatware', labelKey: 'nav.bloatware', icon: <Trash2 size={18} />, section: 'tools' },
  { id: 'history', labelKey: 'nav.history', icon: <History size={18} />, section: 'system' },
  { id: 'settings', labelKey: 'nav.settings', icon: <SettingsIcon size={18} />, section: 'system' },
];

const SECTIONS = [
  { id: 'main', labelKey: 'nav.category_main' },
  { id: 'tools', labelKey: 'nav.category_tools' },
  { id: 'system', labelKey: 'nav.category_system' },
];

function App() {
  const { t, i18n } = useTranslation();
  const [activePage, setActivePage] = useState<Page>('dashboard');

  function renderPage() {
    switch (activePage) {
      case 'dashboard': return <Dashboard />;
      case 'safe_optimize': return <SafeOptimize />;
      case 'advanced': return <AdvancedTweaks />;
      case 'process': return <ProcessManager />;
      case 'bloatware': return <BloatwareRemover />;
      case 'history': return <ActionHistory />;
      case 'settings': return <Settings />;
      default: return <Dashboard />;
    }
  }

  return (
    <div className="app-shell">
      {/* Sidebar */}
      <aside className="sidebar">
        <div className="sidebar-header">
          <div className="sidebar-logo">
            <h1>Estz</h1>
            <span className="version">{t('app.version')}</span>
          </div>
          <p style={{ fontSize: 12, color: 'var(--text-tertiary)', marginTop: 6 }}>{t('app.tagline')}</p>
        </div>

        <nav className="sidebar-nav">
          {SECTIONS.map(section => {
            const sectionItems = NAV_ITEMS.filter(item => item.section === section.id);
            return (
              <div key={section.id}>
                <div className="nav-section-label">{t(section.labelKey)}</div>
                {sectionItems.map(item => (
                  <div
                    key={item.id}
                    className={`nav-item ${activePage === item.id ? 'active' : ''}`}
                    onClick={() => setActivePage(item.id)}
                  >
                    {item.icon}
                    <span>{t(item.labelKey)}</span>
                  </div>
                ))}
              </div>
            );
          })}
        </nav>

        <div className="sidebar-footer">
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <span>© 2026 Estz</span>
            <div className="lang-switch">
              <button
                className={`lang-btn ${i18n.language === 'en' ? 'active' : ''}`}
                onClick={() => { i18n.changeLanguage('en'); localStorage.setItem('estz-language', 'en'); }}
              >
                EN
              </button>
              <button
                className={`lang-btn ${i18n.language === 'th' ? 'active' : ''}`}
                onClick={() => { i18n.changeLanguage('th'); localStorage.setItem('estz-language', 'th'); }}
              >
                TH
              </button>
            </div>
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main className="main-content" key={activePage}>
        {renderPage()}
      </main>
    </div>
  );
}

export default App;
