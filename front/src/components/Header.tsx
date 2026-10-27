interface HeaderProps {
  title?: string
}

export const Header = ({ title = 'Heaven Case' }: HeaderProps) => {
  return (
    <header className="border-b border-neutral-800 bg-neutral-900/60 backdrop-blur-sm px-6 py-4">
      <div className="mx-auto flex max-w-7xl items-center justify-between">
        <h1 className="text-xl font-bold tracking-tight text-white">{title}</h1>
        <span className="text-xs font-medium text-emerald-400 bg-emerald-950/60 border border-emerald-800/50 rounded-full px-2.5 py-0.5">
          v0.0.1
        </span>
      </div>
    </header>
  )
}
