import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Button from "@repo/ui/components/button"

import "./style.css"

const MyFriendView: React.FunctionComponent = () => {
  const hasFriend = false

  const handleRequest = () => {
    // TODO: Implementar solicitud de amigo
  }

  return (
    <section className="MyFriendView">
      <Typo type="title" align="center">Tu amigo</Typo>
      <Divider />

      {!hasFriend && (
        <div className="MyFriendView__empty">
          <Typo align="center">
            Al parecer aún no tienes un amigo adignado, no te preocupes puedes solicitar uno haciendo click abajo.
          </Typo>
          <Button onClick={handleRequest}>Solicitar</Button>
        </div>
      )}
    </section>
  )
}

export default MyFriendView
