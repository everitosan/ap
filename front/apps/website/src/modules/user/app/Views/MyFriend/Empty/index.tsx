import { useState } from "react"
import { useNavigate } from "react-router"
import { FetchError } from "justfetch-ts"
import Typo from "@repo/ui/components/typography"
import Button from "@repo/ui/components/button"
import { useUser } from "@/modules/user/app/UserProvider"
import { requestMatch } from "@/modules/matching/infra/repository/matching"
import type { MatchStatusResponse, ApiError } from "@/api/ap"
import "./style.css"

const MyFriendEmptyView: React.FunctionComponent<{
  onStatusChange: (status: MatchStatusResponse["status"]) => void
}> = ({ onStatusChange }) => {
  const navigate = useNavigate()
  const { user } = useUser()
  const [loading, setLoading] = useState(false)

  const handleRequest = async () => {
    if (!user.filled_address) {
      navigate("/fill-address")
      return
    }

    setLoading(true)
    try {
      const response = await requestMatch()
      onStatusChange(response.data.status)
    } catch (err) {
      if (err instanceof FetchError) {
        const apiError = err.message as unknown as ApiError
        alert(apiError.message || "Error al solicitar amigo")
      } else {
        alert("Error inesperado")
      }
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="MyFriendViewEmpty">
      <Typo align="center">
        Al parecer aún no tienes un amigo asignado, no te preocupes puedes solicitar uno haciendo click abajo.
      </Typo>
      <Button loading={loading} onClick={handleRequest}>Solicitar</Button>
    </div>
  )
}

export default MyFriendEmptyView