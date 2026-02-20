import { useNavigate } from "react-router"
import Typo from "@repo/ui/components/typography"
import Button from "@repo/ui/components/button"
import { useUser } from "@/modules/user/app/UserProvider"
import "./style.css"

const MyFriendEmptyView: React.FunctionComponent = () => {
  const navigate = useNavigate()
  const { user } = useUser()

  const handleRequest = () => {
    if (!user.filled_address) {
      navigate("/fill-address")
      return
    }
    // TODO: Implementar solicitud de amigo
  }

  return (
    <div className="MyFriendViewEmpty">
        <Typo align="center">
          Al parecer aún no tienes un amigo asignado, no te preocupes puedes solicitar uno haciendo click abajo.
        </Typo>
      <Button onClick={handleRequest}>Solicitar</Button>
    </div>
  )
}

export default MyFriendEmptyView