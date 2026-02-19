import { useState, useEffect, createContext, useContext } from "react"
import type { Topic } from "@/modules/catalogues/domain/topic"
import { getTopics } from "@/modules/catalogues/infra/repository/topics"

type CataloguesContextType = {
  topics: Topic[]
}

const CataloguesContext = createContext<CataloguesContextType | null>(null)

export const useCatalogues = () => {
  const context = useContext(CataloguesContext)
  if (!context) {
    throw new Error("useCatalogues must be used within CataloguesProvider")
  }
  return context
}

type CataloguesProviderProps = {
  children: React.ReactNode
}

const CataloguesProvider: React.FunctionComponent<CataloguesProviderProps> = ({ children }) => {
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    Promise.all([
      getTopics()
    ]).then(([topicsResponse]) => {
      setTopics(topicsResponse.data)
      setLoading(false)
    }).catch(() => {
      setLoading(false)
    })
  }, [])

  if (loading) {
    return null
  }

  return (
    <CataloguesContext.Provider value={{ topics }}>
      {children}
    </CataloguesContext.Provider>
  )
}

export default CataloguesProvider
