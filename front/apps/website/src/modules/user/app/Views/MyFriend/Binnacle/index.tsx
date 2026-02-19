import Typo from "@repo/ui/components/typography"
import Button from "@repo/ui/components/button"
import "./style.css"

const MyFriendBinnacleView: React.FunctionComponent = () => {
  const handleAddRecord = () => {
    // TODO: Implementar agregar registro
  }

  const handleFinishInteraction = () => {
    // TODO: Implementar finalizar interacción
  }

  return (
    <div className="MyFriendViewBinnacle">
      <div className="MyFriendViewBinnacle__info">
        <Typo>
          <strong>Amigo:</strong> Everitosan
        </Typo>
        <Typo>
          <strong>Dirección:</strong> Prol 5 de Mayo No 5, int 11, colonia San Andres Totoltepec, CP 14400
        </Typo>
      </div>

      <Typo align="center">
        Ya tienes una dirección y un pseudómino a quién dirigir tu carta. Para dar un mejor seguimiento de la interacción, agrega el registro en cuanto mandes tu carta.
      </Typo>

      <Button onClick={handleAddRecord}>Agregar</Button>

      <button
        className="MyFriendViewBinnacle__finish"
        onClick={handleFinishInteraction}
      >
        Finalizar interacción
      </button>
    </div>
  )
}

export default MyFriendBinnacleView
