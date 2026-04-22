import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { tauriApi, type LaunchConfig, type ConnectionStatus, type GameInfo } from './useTauriApi';

// Connection state from Rust backend
export interface ServerInfo {
  address: string;
  port: number;
  name: string;
  players: number;
  maxPlayers: number;
  gamemode: string;
  version: string;
  ping: number;
}

// App state store
interface AppState {
  // Theme
  theme: 'dark' | 'light';
  setTheme: (theme: 'dark' | 'light') => void;
  
  // Connection
  connected: boolean;
  serverInfo: ServerInfo | null;
  setConnected: (connected: boolean) => void;
  setServerInfo: (info: ServerInfo | null) => void;
  
  // User
  username: string;
  setUsername: (name: string) => void;
  
  // Game
  gameId: 'gta5' | 'rdr3';
  setGameId: (id: 'gta5' | 'rdr3') => void;
  
  // UI
  sidebarOpen: boolean;
  toggleSidebar: () => void;
  
  // Launch options
  launchOptions: {
    server: string;
    port: number;
    authToken: string;
    windowed: boolean;
    skipIntro: boolean;
  };
  setLaunchOptions: (options: Partial<AppState['launchOptions']>) => void;
  
  // Connection actions
  connect: (server: string, port: number, token?: string) => Promise<void>;
  disconnect: () => Promise<void>;
  launchGame: () => Promise<number>;
  detectGames: () => Promise<GameInfo[]>;
}

export const useAppStore = create<AppState>()(
  persist(
    (set, get) => ({
      // Theme
      theme: 'dark',
      setTheme: (theme) => set({ theme }),
      
      // Connection
      connected: false,
      serverInfo: null,
      setConnected: (connected) => set({ connected }),
      setServerInfo: (info) => set({ serverInfo: info }),
      
      // User
      username: '',
      setUsername: (name) => set({ username: name }),
      
      // Game
      gameId: 'gta5',
      setGameId: (id) => set({ gameId: id }),
      
      // UI
      sidebarOpen: false,
      toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      
      // Launch options
      launchOptions: {
        server: 'localhost',
        port: 8080,
        authToken: '',
        windowed: false,
        skipIntro: true,
      },
      setLaunchOptions: (options) => 
        set((state) => ({ 
          launchOptions: { ...state.launchOptions, ...options } 
        })),
      
      // Connection actions
      connect: async (server, port, token) => {
        const config: LaunchConfig = {
          server,
          port,
          token: token || undefined,
          game_id: get().gameId,
          windowed: get().launchOptions.windowed,
          skip_intro: get().launchOptions.skipIntro,
        };
        
        const status = await tauriApi.connectToServer(config);
        set({
          serverInfo: status.server_name ? {
            address: server,
            port,
            name: status.server_name,
            players: status.players || 0,
            maxPlayers: status.max_players || 64,
            gamemode: 'Unknown',
            version: '0.1.0',
            ping: status.ping || 0,
          } : null,
          connected: status.connected,
        });
      },
      
      disconnect: async () => {
        await tauriApi.disconnect();
        set({ connected: false, serverInfo: null });
      },
      
      launchGame: async () => {
        const { launchOptions, gameId } = get();
        const config: LaunchConfig = {
          server: launchOptions.server,
          port: launchOptions.port,
          token: launchOptions.authToken || undefined,
          game_id: gameId,
          windowed: launchOptions.windowed,
          skip_intro: launchOptions.skipIntro,
        };
        
        return await tauriApi.launchGame(config);
      },
      
      detectGames: async () => {
        return await tauriApi.detectGames();
      },
    }),
    {
      name: 'gameverse-storage',
      partialize: (state) => ({
        theme: state.theme,
        username: state.username,
        gameId: state.gameId,
        launchOptions: state.launchOptions,
      }),
    }
  )
);

// API hooks for server communication
export const useServerApi = {
  async getServers(): Promise<ServerInfo[]> {
    // TODO: Fetch from server when connected
    return [
      {
        address: 'play.gameverse.dev',
        port: 8080,
        name: 'GameVerse Official',
        players: 42,
        maxPlayers: 128,
        gamemode: 'Roleplay',
        version: '0.1.0',
        ping: 45,
      },
      {
        address: 'sandbox.gameverse.dev',
        port: 8080,
        name: 'Sandbox',
        players: 5,
        maxPlayers: 32,
        gamemode: 'Sandbox',
        version: '0.1.0',
        ping: 89,
      },
    ];
  },
  
  async checkServer(address: string, port: number): Promise<ServerInfo | null> {
    // TODO: Check server status via API
    return {
      address,
      port,
      name: 'Server',
      players: 0,
      maxPlayers: 64,
      gamemode: 'Unknown',
      version: '0.1.0',
      ping: 0,
    };
  },
};
