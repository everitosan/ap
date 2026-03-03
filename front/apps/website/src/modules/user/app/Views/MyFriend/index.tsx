import { useState, useEffect } from "react"
import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"

// Section components
import Empty from "./Empty"
import WaitingList from "./WaitingList"
import Binnacle from "./Binnacle"

import { getMatchStatus } from "@/modules/matching/infra/repository/matching"
import type { MatchStatusResponse, PartnerData } from "@/api/ap"

import "./style.css"

type MatchState = MatchStatusResponse["status"] | "loading"

const MyFriendView: React.FunctionComponent = () => {
  const [matchState, setMatchState] = useState<MatchState>("loading")
  const [partner, setPartner] = useState<PartnerData | null>(null)

  useEffect(() => {
    getMatchStatus()
      .then((res) => {
        setMatchState(res.data.status)
        setPartner(res.data.partner)
      })
      .catch(() => setMatchState("idle"))
  }, [])

  if (matchState === "loading") {
    return null
  }

  return (
    <section className="MyFriendView">
      <Typo type="title" align="center">Tu amigo</Typo>
      <Divider />

      {matchState === "idle" && (
        <Empty onStatusChange={setMatchState} />
      )}

      {(matchState === "queued" || matchState === "already_queued") && (
        <WaitingList />
      )}

      {(matchState === "paired" || matchState === "already_paired") && partner && (
        <Binnacle partner={partner} onStatusChange={setMatchState} />
      )}
    </section>
  )
}

export default MyFriendView
