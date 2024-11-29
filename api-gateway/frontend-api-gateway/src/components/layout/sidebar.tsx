// src/components/layout/sidebar.tsx
// src/components/layout/sidebar.tsx
interface SidebarProps {
  onPageChange: (page: string) => void;
}

export function Sidebar({ onPageChange }: SidebarProps) {
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: 'dashboard' },
    { id: 'routes', label: 'Routes', icon: 'routes' },
    { id: 'services', label: 'Services', icon: 'services' },
    { id: 'security', label: 'Security', icon: 'security' }
  ];

  return (
    <div className="w-64 border-r bg-card">
      <nav className="space-y-1 p-4">
        {menuItems.map((item) => (
          <button
            key={item.id}
            onClick={() => onPageChange(item.id)}
            className="flex items-center w-full px-3 py-2 text-sm rounded-md hover:bg-accent"
          >
            <span>{item.label}</span>
          </button>
        ))}
      </nav>
    </div>
  );
}