import { motion } from 'framer-motion';
import { useAppStore, useServerApi, ServerInfo } from '../hooks/useAppStore';
import Button from '../components/Button';
import { 
  Play, 
  Users, 
  Signal, 
  Gamepad2,
  Clock,
  Plus
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useState, useEffect } from 'react';

export default function Home() {
  const navigate = useNavigate();
  const { connected, serverInfo, gameId, setGameId, launchOptions, launchGame, setLaunchOptions } = useAppStore();
  const [recentServers] = useState<ServerInfo[]>([]);

  const handleQuickConnect = () => {
    if (connected) {
      navigate('/');
    }
  };

  const handleLaunch = async () => {
    try {
      await launchGame();
    } catch (error) {
      console.error('Launch failed:', error);
    }
  };

  return (
    <div className="space-y-6 max-w-6xl mx-auto">
      {/* Quick Actions */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* Quick Play Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="col-span-1 md:col-span-2 glass rounded-2xl p-6"
        >
          <div className="flex items-start justify-between">
            <div>
              <h2 className="text-2xl font-bold mb-2">Quick Play</h2>
              <p className="text-gray-400 mb-4">
                Connect to your last server and start playing immediately.
              </p>
              
              {connected ? (
                <div className="flex items-center gap-4">
                  <div className="flex-1">
                    <div className="text-sm text-gray-400">Connected to</div>
                    <div className="font-medium">{serverInfo?.name}</div>
                    <div className="text-sm text-gray-500">
                      {serverInfo?.players}/{serverInfo?.maxPlayers} players
                    </div>
                  </div>
                  <Button variant="primary" glow onClick={handleLaunch}>
                    <Play className="w-5 h-5" />
                    Resume Game
                  </Button>
                </div>
              ) : (
                <div className="flex items-center gap-4">
                  <div className="flex-1">
                    <div className="text-sm text-gray-400">Server</div>
                    <div className="font-medium">
                      {launchOptions.server}:{launchOptions.port}
                    </div>
                  </div>
                  <Button variant="primary" glow onClick={handleQuickConnect}>
                    <Play className="w-5 h-5" />
                    Connect
                  </Button>
                </div>
              )}
            </div>
          </div>
        </motion.div>

        {/* Game Selector */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="glass rounded-2xl p-6"
        >
          <h3 className="text-lg font-medium mb-4">Select Game</h3>
          <div className="space-y-2">
            <button
              onClick={() => setGameId('gta5')}
              className={`w-full flex items-center gap-3 p-3 rounded-lg transition-all ${
                gameId === 'gta5' 
                  ? 'bg-gv-primary/10 border border-gv-primary text-gv-primary' 
                  : 'bg-gv-darker border border-gv-border hover:border-gv-primary/50'
              }`}
            >
              <Gamepad2 className="w-5 h-5" />
              <div className="text-left">
                <div className="font-medium">GTA V</div>
                <div className="text-xs text-gray-400">Grand Theft Auto V</div>
              </div>
            </button>
            <button
              onClick={() => setGameId('rdr3')}
              className={`w-full flex items-center gap-3 p-3 rounded-lg transition-all ${
                gameId === 'rdr3' 
                  ? 'bg-gv-primary/10 border border-gv-primary text-gv-primary' 
                  : 'bg-gv-darker border border-gv-border hover:border-gv-primary/50'
              }`}
            >
              <Gamepad2 className="w-5 h-5" />
              <div className="text-left">
                <div className="font-medium">RDR3</div>
                <div className="text-xs text-gray-400">Red Dead Redemption 2</div>
              </div>
            </button>
          </div>
        </motion.div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {[
          { label: 'Servers Online', value: '24', icon: Signal },
          { label: 'Total Players', value: '1,247', icon: Users },
          { label: 'Active Games', value: '2', icon: Gamepad2 },
          { label: 'Uptime', value: '99.9%', icon: Clock },
        ].map((stat, i) => (
          <motion.div
            key={stat.label}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 + i * 0.05 }}
            className="glass rounded-xl p-4"
          >
            <stat.icon className="w-5 h-5 text-gv-primary mb-2" />
            <div className="text-2xl font-bold">{stat.value}</div>
            <div className="text-sm text-gray-400">{stat.label}</div>
          </motion.div>
        ))}
      </div>

      {/* Recent Servers */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="glass rounded-2xl p-6"
      >
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium">Recent Servers</h3>
          <Button variant="ghost" size="sm">
            <Plus className="w-4 h-4" />
            Add Server
          </Button>
        </div>
        
        {recentServers.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            <Signal className="w-12 h-12 mx-auto mb-4 opacity-30" />
            <p>No recent servers</p>
            <p className="text-sm">Connect to a server to see it here</p>
          </div>
        ) : (
          <div className="space-y-2">
            {recentServers.map((server) => (
              <ServerCard key={server.address} server={server} />
            ))}
          </div>
        )}
      </motion.div>
    </div>
  );
}

// Server Card Component
function ServerCard({ server }: { server: ServerInfo }) {
  const { connect, setLaunchOptions, setGameId } = useAppStore();
  const [loading, setLoading] = useState(false);

  const handleConnect = async () => {
    setLoading(true);
    try {
      setGameId('gta5'); // Default
      setLaunchOptions({ server: server.address, port: server.port });
      await connect(server.address, server.port);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex items-center gap-4 p-4 bg-gv-darker rounded-lg border border-gv-border hover:border-gv-primary/30 transition-colors">
      <div className="w-12 h-12 rounded-lg bg-gradient-to-br from-gv-primary/20 to-gv-secondary/20 flex items-center justify-center">
        <Signal className="w-6 h-6 text-gv-primary" />
      </div>
      
      <div className="flex-1">
        <div className="font-medium">{server.name}</div>
        <div className="text-sm text-gray-400">
          {server.address}:{server.port} • {server.gamemode}
        </div>
      </div>
      
      <div className="text-right">
        <div className="flex items-center gap-1 text-sm">
          <Users className="w-4 h-4 text-gray-400" />
          <span>{server.players}/{server.maxPlayers}</span>
        </div>
        <div className="text-sm text-gray-500">{server.ping}ms</div>
      </div>
      
      <Button variant="primary" size="sm" loading={loading} onClick={handleConnect}>
        <Play className="w-4 h-4" />
        Connect
      </Button>
    </div>
  );
}
