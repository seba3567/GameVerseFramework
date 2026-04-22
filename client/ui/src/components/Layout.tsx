import { Outlet, NavLink, useLocation } from 'react-router-dom';
import { useAppStore } from '../hooks/useAppStore';
import { 
  Home, 
  Server, 
  Settings, 
  LogOut, 
  ChevronLeft, 
  ChevronRight,
  Gamepad2,
  Wifi,
  WifiOff
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

export default function Layout() {
  const location = useLocation();
  const { 
    sidebarOpen, 
    toggleSidebar, 
    connected, 
    serverInfo,
    disconnect 
  } = useAppStore();

  const navItems = [
    { to: '/', icon: Home, label: 'Home' },
    { to: '/servers', icon: Server, label: 'Servers' },
    { to: '/settings', icon: Settings, label: 'Settings' },
  ];

  return (
    <div className="flex h-screen bg-gv-darker">
      {/* Sidebar */}
      <motion.aside
        initial={false}
        animate={{ width: sidebarOpen ? 240 : 72 }}
        className="bg-gv-surface border-r border-gv-border flex flex-col"
      >
        {/* Logo */}
        <div className="h-16 flex items-center px-4 border-b border-gv-border" data-tauri-drag-region>
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-gv-primary to-gv-secondary flex items-center justify-center">
              <Gamepad2 className="w-6 h-6 text-gv-dark" />
            </div>
            <AnimatePresence>
              {sidebarOpen && (
                <motion.span
                  initial={{ opacity: 0, width: 0 }}
                  animate={{ opacity: 1, width: 'auto' }}
                  exit={{ opacity: 0, width: 0 }}
                  className="font-bold text-lg whitespace-nowrap overflow-hidden"
                >
                  GameVerse
                </motion.span>
              )}
            </AnimatePresence>
          </div>
        </div>

        {/* Navigation */}
        <nav className="flex-1 py-4 px-3 space-y-2">
          {navItems.map(({ to, icon: Icon, label }) => (
            <NavLink
              key={to}
              to={to}
              className={({ isActive }) =>
                clsx(
                  'flex items-center gap-3 px-3 py-3 rounded-lg transition-all',
                  'hover:bg-gv-border/50',
                  isActive && 'bg-gv-primary/10 text-gv-primary',
                  !isActive && 'text-gray-400'
                )
              }
            >
              <Icon className="w-5 h-5 flex-shrink-0" />
              <AnimatePresence>
                {sidebarOpen && (
                  <motion.span
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="whitespace-nowrap"
                  >
                    {label}
                  </motion.span>
                )}
              </AnimatePresence>
            </NavLink>
          ))}
        </nav>

        {/* Connection Status */}
        <div className="px-3 py-4 border-t border-gv-border">
          <div 
            className={clsx(
              'flex items-center gap-3 px-3 py-2 rounded-lg',
              connected ? 'bg-green-500/10 text-green-400' : 'bg-red-500/10 text-red-400'
            )}
          >
            {connected ? <Wifi className="w-5 h-5" /> : <WifiOff className="w-5 h-5" />}
            <AnimatePresence>
              {sidebarOpen && (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="flex-1"
                >
                  <div className="text-sm font-medium">
                    {connected ? serverInfo?.name || 'Connected' : 'Disconnected'}
                  </div>
                  {connected && serverInfo && (
                    <div className="text-xs opacity-70">
                      {serverInfo.players}/{serverInfo.maxPlayers} players
                    </div>
                  )}
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>

        {/* Toggle Button */}
        <button
          onClick={toggleSidebar}
          className="absolute -right-3 top-20 w-6 h-6 bg-gv-surface border border-gv-border rounded-full flex items-center justify-center hover:bg-gv-border transition-colors"
        >
          {sidebarOpen ? <ChevronLeft className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
        </button>
      </motion.aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Top Bar */}
        <header className="h-14 bg-gv-surface border-b border-gv-border flex items-center justify-between px-6" data-tauri-drag-region>
          <div className="text-sm text-gray-400">
            {location.pathname === '/' && 'Dashboard'}
            {location.pathname === '/servers' && 'Server Browser'}
            {location.pathname === '/settings' && 'Settings'}
          </div>
          
          {connected && (
            <button
              onClick={disconnect}
              className="flex items-center gap-2 px-4 py-2 rounded-lg bg-red-500/10 text-red-400 hover:bg-red-500/20 transition-colors"
            >
              <LogOut className="w-4 h-4" />
              <span className="text-sm">Disconnect</span>
            </button>
          )}
        </header>

        {/* Content Area */}
        <div className="flex-1 overflow-auto p-6">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
