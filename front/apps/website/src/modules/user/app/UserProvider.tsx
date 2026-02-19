import { createContext, useContext, useState } from "react"
import type { UserResponse } from "@/api/ap"

type UserContextType = {
  user: UserResponse
  updateUser: (updates: Partial<UserResponse>) => void
}

const UserContext = createContext<UserContextType | null>(null)

export const useUser = () => {
  const context = useContext(UserContext)
  if (!context) {
    throw new Error("useUser must be used within UserProvider")
  }
  return context
}

type UserProviderProps = {
  user: UserResponse
  children: React.ReactNode
}

const UserProvider: React.FunctionComponent<UserProviderProps> = ({ user: initialUser, children }) => {
  const [user, setUser] = useState<UserResponse>(initialUser)

  const updateUser = (updates: Partial<UserResponse>) => {
    setUser(prev => ({ ...prev, ...updates }))
  }

  return (
    <UserContext.Provider value={{ user, updateUser }}>
      {children}
    </UserContext.Provider>
  )
}

export default UserProvider
