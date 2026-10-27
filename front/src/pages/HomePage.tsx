import { Header } from '../components/Header'

export const HomePage = () => {
  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 flex flex-col">
      <Header />
      <main className="flex-1 flex flex-col items-center justify-center p-6 text-center">
        <h2 className="text-3xl font-extrabold tracking-tight sm:text-4xl text-white">
          Bienvenue sur Heaven Case
        </h2>
        <p className="mt-3 max-w-md text-neutral-400 text-sm sm:text-base">
          Plateforme d&apos;ouverture de caisses virtuelles. Projet React configuré avec Tailwind CSS v4.
        </p>
      </main>
    </div>
  )
}
