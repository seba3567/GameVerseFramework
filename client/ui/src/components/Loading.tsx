import { motion } from 'framer-motion';
import { Loader2 } from 'lucide-react';
import clsx from 'clsx';

interface LoadingProps {
  full?: boolean;
  size?: 'sm' | 'md' | 'lg';
  text?: string;
}

export default function Loading({ full = false, size = 'md', text = 'Loading...' }: LoadingProps) {
  const sizes = {
    sm: 'w-5 h-5',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  if (full) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gv-darker">
        <div className="flex flex-col items-center gap-4">
          <motion.div
            animate={{ rotate: 360 }}
            transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
            className="w-16 h-16 rounded-full border-4 border-gv-border border-t-gv-primary"
          />
          <p className="text-gray-400">{text}</p>
        </div>
      </div>
    );
  }

  return (
    <div className={clsx('flex items-center gap-3', full && 'justify-center')}>
      <motion.div
        animate={{ rotate: 360 }}
        transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
        className={clsx(
          'rounded-full border-4 border-gv-border border-t-gv-primary',
          sizes[size]
        )}
      />
      {text && <span className="text-gray-400">{text}</span>}
    </div>
  );
}
