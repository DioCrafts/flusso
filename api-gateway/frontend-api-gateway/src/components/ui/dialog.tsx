// src/components/ui/dialog.tsx
import React from 'react';

interface DialogProps {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  children: React.ReactNode;
  className?: string;
}

export function Dialog({ open, onOpenChange, children, className = '' }: DialogProps) {
  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm">
      <div className="fixed inset-0 z-50 flex items-center justify-center">
        <div className={`relative bg-background p-6 shadow-lg rounded-lg ${className}`}>
          {children}
          <button
            className="absolute top-4 right-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100"
            onClick={() => onOpenChange?.(false)}
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  );
}

export function DialogContent({ children, className = '' }: { children: React.ReactNode; className?: string }) {
  return (
    <div className={`max-w-lg mx-auto ${className}`}>
      {children}
    </div>
  );
}

export function DialogHeader({ children, className = '' }: { children: React.ReactNode; className?: string }) {
  return (
    <div className={`mb-4 ${className}`}>
      {children}
    </div>
  );
}

export function DialogTitle({ children, className = '' }: { children: React.ReactNode; className?: string }) {
  return (
    <h2 className={`text-lg font-semibold leading-none tracking-tight ${className}`}>
      {children}
    </h2>
  );
}
