import { motion } from 'framer-motion';
import { useAppStore } from '../hooks/useAppStore';
import Button from '../components/Button';
import Input from '../components/Input';
import { 
  Monitor, 
  Keyboard, 
  Globe, 
  Palette, 
  Gamepad2,
  Info,
  Github,
  ExternalLink
} from 'lucide-react';
import { useState } from 'react';

export default function Settings() {
  const { theme, setTheme, gameId, launchOptions, setLaunchOptions } = useAppStore();
  const [localOptions, setLocalOptions] = useState(launchOptions);

  const handleSave = () => {
    setLaunchOptions(localOptions);
  };

  const sections = [
    {
      title: 'Display',
      icon: Monitor,
      items: [
        {
          label: 'Windowed Mode',
          description: 'Launch game in windowed mode instead of fullscreen',
          type: 'toggle',
          value: localOptions.windowed,
          onChange: (v: boolean) => setLocalOptions({ ...localOptions, windowed: v }),
        },
        {
          label: 'Skip Intro Videos',
          description: 'Skip startup videos and logos',
          type: 'toggle',
          value: localOptions.skipIntro,
          onChange: (v: boolean) => setLocalOptions({ ...localOptions, skipIntro: v }),
        },
      ],
    },
    {
      title: 'Game',
      icon: Gamepad2,
      items: [
        {
          label: 'Default Game',
          description: 'Game to launch by default',
          type: 'select',
          value: gameId,
          options: [
            { value: 'gta5', label: 'Grand Theft Auto V' },
            { value: 'rdr3', label: 'Red Dead Redemption 2' },
          ],
          onChange: (v: string) => useAppStore.getState().setGameId(v as 'gta5' | 'rdr3'),
        },
      ],
    },
    {
      title: 'Network',
      icon: Globe,
      items: [
        {
          label: 'Default Server',
          description: 'Server to connect to by default',
          type: 'input',
          value: localOptions.server,
          onChange: (v: string) => setLocalOptions({ ...localOptions, server: v }),
        },
        {
          label: 'Default Port',
          description: 'Server port to connect to by default',
          type: 'input',
          value: String(localOptions.port),
          onChange: (v: string) => setLocalOptions({ ...localOptions, port: parseInt(v) || 8080 }),
        },
      ],
    },
    {
      title: 'Appearance',
      icon: Palette,
      items: [
        {
          label: 'Theme',
          description: 'Choose your preferred color theme',
          type: 'select',
          value: theme,
          options: [
            { value: 'dark', label: 'Dark' },
            { value: 'light', label: 'Light' },
          ],
          onChange: (v: string) => setTheme(v as 'dark' | 'light'),
        },
      ],
    },
  ];

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      {/* Settings Sections */}
      {sections.map((section, i) => (
        <motion.div
          key={section.title}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: i * 0.05 }}
          className="glass rounded-2xl overflow-hidden"
        >
          <div className="px-6 py-4 border-b border-gv-border flex items-center gap-3">
            <section.icon className="w-5 h-5 text-gv-primary" />
            <h2 className="font-semibold">{section.title}</h2>
          </div>
          
          <div className="p-6 space-y-4">
            {section.items.map((item) => (
              <div key={item.label} className="flex items-center justify-between py-2">
                <div className="flex-1">
                  <div className="font-medium">{item.label}</div>
                  <div className="text-sm text-gray-400">{item.description}</div>
                </div>
                
                <div className="ml-4">
                  {item.type === 'toggle' && (
                    <button
                      onClick={() => item.onChange?.(!item.value)}
                      className={`w-12 h-7 rounded-full transition-colors ${
                        item.value ? 'bg-gv-primary' : 'bg-gv-border'
                      }`}
                    >
                      <div className={`w-5 h-5 bg-white rounded-full shadow transition-transform ${
                        item.value ? 'translate-x-6' : 'translate-x-1'
                      }`} />
                    </button>
                  )}
                  
                  {item.type === 'input' && (
                    <input
                      type="text"
                      value={item.value}
                      onChange={(e) => item.onChange?.(e.target.value)}
                      className="w-40 bg-gv-darker border border-gv-border rounded-lg px-3 py-1.5 text-sm text-white focus:outline-none focus:ring-2 focus:ring-gv-primary/50"
                    />
                  )}
                  
                  {item.type === 'select' && (
                    <select
                      value={item.value}
                      onChange={(e) => item.onChange?.(e.target.value)}
                      className="bg-gv-darker border border-gv-border rounded-lg px-3 py-1.5 text-sm text-white focus:outline-none focus:ring-2 focus:ring-gv-primary/50"
                    >
                      {item.options?.map((opt) => (
                        <option key={opt.value} value={opt.value}>
                          {opt.label}
                        </option>
                      ))}
                    </select>
                  )}
                </div>
              </div>
            ))}
          </div>
        </motion.div>
      ))}

      {/* Save Button */}
      <div className="flex justify-end">
        <Button variant="primary" onClick={handleSave}>
          Save Changes
        </Button>
      </div>

      {/* About */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
        className="glass rounded-2xl p-6"
      >
        <div className="flex items-center gap-3 mb-4">
          <Info className="w-5 h-5 text-gv-primary" />
          <h2 className="font-semibold">About</h2>
        </div>
        
        <div className="space-y-2 text-sm text-gray-400">
          <p>GameVerse Client v0.1.0</p>
          <p>Built with Rust + React + Tauri</p>
          <div className="flex items-center gap-4 pt-2">
            <a 
              href="https://github.com/gameverse/GameVerseFramework" 
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-1 text-gv-primary hover:underline"
            >
              <Github className="w-4 h-4" />
              GitHub
              <ExternalLink className="w-3 h-3" />
            </a>
          </div>
        </div>
      </motion.div>
    </div>
  );
}
