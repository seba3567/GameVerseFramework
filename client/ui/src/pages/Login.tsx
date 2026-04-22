import { motion } from 'framer-motion';
import { useNavigate } from 'react-router-dom';
import { useAppStore } from '../hooks/useAppStore';
import Button from '../components/Button';
import { useState } from 'react';
import { LogIn, User, Lock, Gamepad2 } from 'lucide-react';

export default function Login() {
  const navigate = useNavigate();
  const { setUsername, username, connect, launchOptions } = useAppStore();
  const [loading, setLoading] = useState(false);
  const [formData, setFormData] = useState({
    username: '',
    server: launchOptions.server,
    port: String(launchOptions.port),
    token: '',
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    
    try {
      setUsername(formData.username);
      await connect(formData.server, parseInt(formData.port), formData.token);
      navigate('/');
    } catch (error) {
      console.error('Connection failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-gv-darker via-gv-dark to-gv-surface p-4">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="w-full max-w-md"
      >
        {/* Logo */}
        <div className="text-center mb-8">
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ delay: 0.2, type: 'spring' }}
            className="w-20 h-20 mx-auto rounded-2xl bg-gradient-to-br from-gv-primary to-gv-secondary flex items-center justify-center mb-4 shadow-lg shadow-gv-primary/30"
          >
            <Gamepad2 className="w-10 h-10 text-gv-dark" />
          </motion.div>
          <h1 className="text-3xl font-bold bg-gradient-to-r from-gv-primary to-gv-secondary bg-clip-text text-transparent">
            GameVerse
          </h1>
          <p className="text-gray-400 mt-2">Connect to multiplayer servers</p>
        </div>

        {/* Login Form */}
        <motion.form
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          onSubmit={handleSubmit}
          className="glass rounded-2xl p-6 space-y-5"
        >
          {/* Username */}
          <div className="space-y-2">
            <label className="text-sm font-medium text-gray-300 flex items-center gap-2">
              <User className="w-4 h-4" />
              Username
            </label>
            <input
              type="text"
              value={formData.username}
              onChange={(e) => setFormData({ ...formData, username: e.target.value })}
              placeholder="Enter your username"
              className="w-full bg-gv-darker border border-gv-border rounded-lg px-4 py-3 text-white placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-gv-primary/50 focus:border-gv-primary transition-all"
              required
            />
          </div>

          {/* Server */}
          <div className="space-y-2">
            <label className="text-sm font-medium text-gray-300">Server Address</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={formData.server}
                onChange={(e) => setFormData({ ...formData, server: e.target.value })}
                placeholder="Server IP or domain"
                className="flex-1 bg-gv-darker border border-gv-border rounded-lg px-4 py-3 text-white placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-gv-primary/50 focus:border-gv-primary transition-all"
                required
              />
              <input
                type="number"
                value={formData.port}
                onChange={(e) => setFormData({ ...formData, port: e.target.value })}
                placeholder="Port"
                className="w-24 bg-gv-darker border border-gv-border rounded-lg px-4 py-3 text-white placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-gv-primary/50 focus:border-gv-primary transition-all"
                required
              />
            </div>
          </div>

          {/* Token (optional) */}
          <div className="space-y-2">
            <label className="text-sm font-medium text-gray-300 flex items-center gap-2">
              <Lock className="w-4 h-4" />
              Auth Token (optional)
            </label>
            <input
              type="password"
              value={formData.token}
              onChange={(e) => setFormData({ ...formData, token: e.target.value })}
              placeholder="Enter auth token"
              className="w-full bg-gv-darker border border-gv-border rounded-lg px-4 py-3 text-white placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-gv-primary/50 focus:border-gv-primary transition-all"
            />
          </div>

          {/* Submit */}
          <Button
            type="submit"
            variant="primary"
            size="lg"
            loading={loading}
            glow
            className="w-full mt-4"
          >
            <LogIn className="w-5 h-5" />
            Connect
          </Button>
        </motion.form>

        {/* Footer */}
        <p className="text-center text-gray-500 text-sm mt-6">
          v0.1.0 • GameVerse Framework
        </p>
      </motion.div>
    </div>
  );
}
