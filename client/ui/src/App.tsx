import { Routes, Route } from 'react-router-dom';
import { lazy, Suspense } from 'react';
import { useAppStore } from './hooks/useAppStore';

// Lazy load pages
const Login = lazy(() => import('./pages/Login'));
const Home = lazy(() => import('./pages/Home'));
const ServerList = lazy(() => import('./pages/ServerList'));
const Settings = lazy(() => import('./pages/Settings'));
const Loading = lazy(() => import('./components/Loading'));

// Layout component
import Layout from './components/Layout';

export default function App() {
  const { theme } = useAppStore();

  return (
    <div className={theme} data-theme={theme}>
      <Suspense fallback={<Loading full />}>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route element={<Layout />}>
            <Route path="/" element={<Home />} />
            <Route path="/servers" element={<ServerList />} />
            <Route path="/settings" element={<Settings />} />
          </Route>
        </Routes>
      </Suspense>
    </div>
  );
}
