import Typo from "@repo/ui/components/typography"
import "./style.css"

const MyFriendWaitingListView: React.FunctionComponent = () => {
  return (
    <div className="MyFriendViewWaitingList">
      <Typo align="center">
        Has ingresado en una lista de espera temporal.
      </Typo>
      <Typo align="center">
        No debería demorar mucho para que encontremos a tu próximo amigo postal.
      </Typo>
    </div>
  )
}

export default MyFriendWaitingListView
