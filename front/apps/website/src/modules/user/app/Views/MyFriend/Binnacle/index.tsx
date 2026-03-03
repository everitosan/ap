import { useState } from "react"
import Typo from "@repo/ui/components/typography"
import Button from "@repo/ui/components/button"
import { cancelMatch } from "@/modules/matching/infra/repository/matching"
import type { PartnerData, AddressData, MatchStatusResponse } from "@/api/ap"
import "./style.css"

const formatAddress = (address: AddressData) => {
  const parts = [
    address.street,
    address.int_number && `int ${address.int_number}`,
    address.colony && `colonia ${address.colony}`,
    address.city,
    address.state,
    `CP ${address.postal_code}`,
  ].filter(Boolean)
  return parts.join(", ")
}

const MyFriendBinnacleView: React.FunctionComponent<{
  partner: PartnerData
  onStatusChange: (status: MatchStatusResponse["status"]) => void
}> = ({ partner, onStatusChange }) => {
  const [loading, setLoading] = useState(false)

  const handleAddRecord = () => {
    // TODO: Implementar agregar registro
  }

  const handleFinishInteraction = async () => {
    setLoading(true)
    try {
      await cancelMatch()
      onStatusChange("idle")
    } catch {
      alert("Error al finalizar la interacción")
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="MyFriendViewBinnacle">
      <div className="MyFriendViewBinnacle__info">
        <Typo>
          <strong>Amigo:</strong> {partner.username}
        </Typo>
        <Typo>
          <strong>Dirección:</strong> {formatAddress(partner.address!)}
        </Typo>
      </div>

      <Typo align="center">
        Ya tienes una dirección y un pseudónimo a quién dirigir tu carta. Para dar un mejor seguimiento de la interacción, agrega el registro en cuanto mandes tu carta.
      </Typo>

      <div className="MyFriendViewBinnacle__buttons" >
        <Button onClick={handleAddRecord}>Agregar</Button>

        <button
          className="MyFriendViewBinnacle__finish"
          onClick={handleFinishInteraction}
          disabled={loading}
        >
          {loading ? "Finalizando..." : "Finalizar interacción"}
        </button>
      </div>
    </div>
  )
}

export default MyFriendBinnacleView
