import { useState, useEffect } from "react"
import PublicRoutes from "./public";
import PrivateRoutes from "./private";
import { api, type UserResponse } from "@/api/ap"
import UserProvider from "@/modules/user/app/UserProvider"

const Routes: React.FunctionComponent = () => {
  const [user, setUser] = useState<UserResponse | null | false>(null)

  useEffect(() => {
    api.getUser()
      .then((res) => setUser(res.data))
      .catch(() => setUser(false))
  }, [])

  if (user === null) {
    return null
  }

  if (user) {
    return (
      <UserProvider user={user}>
        <PrivateRoutes />
      </UserProvider>
    )
  }

  return <PublicRoutes />
}

export default Routes