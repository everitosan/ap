import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Empty from "./Empty"

import "./style.css"

const MyFriendView: React.FunctionComponent = () => {
  const hasFriend = false

  return (
    <section className="MyFriendView">
      <Typo type="title" align="center">Tu amigo</Typo>
      <Divider />

      {!hasFriend && (
        <Empty />
      )}
    </section>
  )
}

export default MyFriendView
