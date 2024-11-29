// src/components/ui/tabs.tsx
import React from 'react';

interface TabsProps {
  defaultValue: string;
  children: React.ReactNode;
  className?: string;
}

export function Tabs({ defaultValue, children, className = '' }: TabsProps) {
  const [value, setValue] = React.useState(defaultValue);

  return (
    <div className={className}>
      {React.Children.map(children, child => {
        if (React.isValidElement(child)) {
          return React.cloneElement(child, { value, onChange: setValue });
        }
        return child;
      })}
    </div>
  );
}

interface TabsListProps {
  children: React.ReactNode;
  className?: string;
}

export function TabsList({ children, className = '' }: TabsListProps) {
  return (
    <div className={`flex space-x-2 border-b ${className}`}>
      {children}
    </div>
  );
}

interface TabsTriggerProps {
  value: string;
  children: React.ReactNode;
  className?: string;
  onChange?: (value: string) => void;
}

export function TabsTrigger({ value, children, className = '', onChange }: TabsTriggerProps) {
  const handleClick = () => {
    onChange?.(value);
  };

  return (
    <button
      onClick={handleClick}
      className={`px-4 py-2 text-sm font-medium transition-colors 
        hover:bg-muted focus:outline-none focus:ring-2 
        focus:ring-ring focus:ring-offset-2
        data-[state=active]:bg-background data-[state=active]:text-foreground
        ${className}`}
      data-state={onChange ? 'active' : 'inactive'}
    >
      {children}
    </button>
  );
}

interface TabsContentProps {
  value: string;
  children: React.ReactNode;
  className?: string;
}

export function TabsContent({ value, children, className = '' }: TabsContentProps & { value: string }) {
  return (
    <div
      role="tabpanel"
      className={`mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ${className}`}
      data-state={value ? 'active' : 'inactive'}
    >
      {children}
    </div>
  );
}
