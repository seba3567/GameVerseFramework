import { motion } from 'framer-motion';
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAppStore, useServerApi, ServerInfo } from '../hooks/useAppStore';
import Button from '../components/Button';
import Input from '../components/Input';
import Loading from '../components/Loading';
import { 
  Search, 
  RefreshCw, 
  Play, 
  Users, 
  Signal,
  Filter,
  ChevronDown,
  X
} from 'lucide-react';

export default function ServerList() {
  const navigate = useNavigate();
  const { setLaunchOptions, connect, setGameId } = useAppStore();
  const [servers, setServers] = useState<ServerInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [search, setSearch] = useState('');
  const [filterOpen, setFilterOpen] = useState(false);

  useEffect(() => {
    loadServers();
  }, []);

  const loadServers = async () => {
    setLoading(true);
    try {
      const data = await useServerApi.getServers();
      setServers(data);
    } catch (error) {
      console.error('Failed to load servers:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRefresh = async () => {
    setRefreshing(true);
    await loadServers();
    setRefreshing(false);
  };

  const handleConnect = async (server: ServerInfo) => {
    setGameId('gta5');
    setLaunchOptions({ server: server.address, port: server.port });
    await connect(server.address, server.port);
    navigate('/');
  };

  const filteredServers = servers.filter(
    (s) =>
      s.name.toLowerCase().includes(search.toLowerCase()) ||
      s.address.toLowerCase().includes(search.toLowerCase()) ||
      s.gamemode.toLowerCase().includes(search.toLowerCase())
  );

  if (loading) {
    return <Loading full text="Loading servers..." />;
  }

  return (
    <div className="space-y-6">
      {/* Search and Filter */}
      <div className="flex items-center gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search servers by name, IP, or gamemode..."
            className="w-full bg-gv-surface border border-gv-border rounded-lg pl-10 pr-4 py-2.5 text-white placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-gv-primary/50 focus:border-gv-primary transition-all"
          />
          {search && (
            <button
              onClick={() => setSearch('')}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-white"
            >
              <X className="w-5 h-5" />
            </button>
          )}
        </div>
        
        <Button 
          variant="secondary" 
          onClick={handleRefresh}
          loading={refreshing}
        >
          <RefreshCw className={`w-4 h-4 ${refreshing ? 'animate-spin' : ''}`} />
          Refresh
        </Button>
        
        <Button 
          variant="secondary"
          onClick={() => setFilterOpen(!filterOpen)}
        >
          <Filter className="w-4 h-4" />
          Filter
          <ChevronDown className={`w-4 h-4 transition-transform ${filterOpen ? 'rotate-180' : ''}`} />
        </Button>
      </div>

      {/* Server List */}
      <div className="space-y-3">
        {filteredServers.length === 0 ? (
          <div className="text-center py-12 text-gray-500 glass rounded-2xl">
            <Signal className="w-12 h-12 mx-auto mb-4 opacity-30" />
            <p>No servers found</p>
            {search && (
              <p className="text-sm">Try a different search term</p>
            )}
          </div>
        ) : (
          filteredServers.map((server, i) => (
            <ServerRow
              key={server.address}
              server={server}
              index={i}
              onConnect={() => handleConnect(server)}
            />
          ))
        )}
      </div>
    </div>
  );
}

function ServerRow({ 
  server, 
  index, 
  onConnect 
}: { 
  server: ServerInfo; 
  index: number; 
  onConnect: () => void;
}) {
  const [connecting, setConnecting] = useState(false);

  const handleConnect = async () => {
    setConnecting(true);
    try {
      await onConnect();
    } finally {
      setConnecting(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.05 }}
      className="glass rounded-xl p-4 flex items-center gap-4 hover:border-gv-primary/30 transition-colors group"
    >
      {/* Icon */}
      <div className="w-14 h-14 rounded-xl bg-gradient-to-br from-gv-primary/20 to-gv-secondary/20 flex items-center justify-center group-hover:from-gv-primary/30 group-hover:to-gv-secondary/30 transition-all">
        <Signal className="w-7 h-7 text-gv-primary" />
      </div>
      
      {/* Info */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <h3 className="font-semibold text-lg truncate">{server.name}</h3>
          <span className="px-2 py-0.5 text-xs rounded-full bg-gv-primary/10 text-gv-primary">
            v{server.version}
          </span>
        </div>
        <div className="flex items-center gap-4 text-sm text-gray-400 mt-1">
          <span className="font-mono">{server.address}:{server.port}</span>
          <span className="flex items-center gap-1">
            <Users className="w-3.5 h-3.5" />
            {server.players}/{server.maxPlayers}
          </span>
          <span>{server.gamemode}</span>
        </div>
      </div>
      
      {/* Ping Indicator */}
      <div className="text-center">
        <div className={`text-lg font-semibold ${
          server.ping < 50 ? 'text-green-400' :
          server.ping < 100 ? 'text-yellow-400' :
          'text-red-400'
        }`}>
          {server.ping}
        </div>
        <div className="text-xs text-gray-500">ms</div>
      </div>
      
      {/* Connect Button */}
      <Button 
        variant="primary" 
        onClick={handleConnect}
        loading={connecting}
      >
        <Play className="w-4 h-4" />
        Connect
      </Button>
    </motion.div>
  );
}
