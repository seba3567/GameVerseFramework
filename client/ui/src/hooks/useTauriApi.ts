import { invoke } from '@tauri-apps/api';

// Types matching Rust structs
export interface LaunchConfig {
  server: string;
  port: number;
  token?: string;
  game_id: string;
  windowed: boolean;
  skip_intro: boolean;
}

export interface ConnectionStatus {
  connected: boolean;
  server_name?: string;
  players?: number;
  max_players?: number;
  ping?: number;
}

export interface GameInfo {
  id: string;
  name: string;
  path?: string;
  detected: boolean;
}

// Tauri API wrapper
export const tauriApi = {
  async launchGame(config: LaunchConfig): Promise<number> {
    return invoke<number>('launch_game', { config });
  },

  async injectDll(pid: number, dllPath: string): Promise<boolean> {
    return invoke<boolean>('inject_dll', { pid, dllPath });
  },

  async getConnectionStatus(): Promise<ConnectionStatus> {
    return invoke<ConnectionStatus>('get_connection_status');
  },

  async connectToServer(config: LaunchConfig): Promise<ConnectionStatus> {
    return invoke<ConnectionStatus>('connect_to_server', { config });
  },

  async disconnect(): Promise<void> {
    return invoke<void>('disconnect');
  },

  async detectGames(): Promise<GameInfo[]> {
    return invoke<GameInfo[]>('detect_games');
  },

  async getGamePath(gameId: string): Promise<string | null> {
    return invoke<string | null>('get_game_path', { gameId });
  },
};
