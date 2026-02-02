import { useState, useEffect } from "react"
import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Estampa2 from "../../../assets/estampa-2.png"
import Input from "@repo/ui/components/input"
import Button from "@repo/ui/components/button"

import "./style.css"

const RetryButton: React.FunctionComponent<{
  onClick: () => void
  count?: number
}> = ({onClick, count=10}) => {
  const [counter, setCounter] = useState<number>(count)

  useEffect(() => {
    const interval = setInterval(() => {
      if (counter > 0) setCounter(counter-1)
    }, 1000)

    return () => clearInterval(interval)

  }, [counter])


  const canClick = () => {
    if (counter === 0)  {
      onClick()
      setCounter(count)
    }
  }

  return (
    <Button onClick={canClick} variant="text" > 
      Reenvíar código  {counter > 0 && ( <span> ({counter}) </span> )} 
    </Button>
  )
}

const ValidateView:React.FunctionComponent = () => {

  const resendCode = () => {
    console.log("resending")
  }

  const tryCode = () => {
    console.log("trying code")
  }

  return (
    <section className="ValidteView">
      <div className="ValidteView__head">
        <Typo type="title" align="center" > Valida tu número </Typo>
        <Divider />
        <img className="ValidteView__estampa" src={Estampa2} />
      </div>
      <div className="ValidteView__validate">
        <Typo> Ingresa el código que recibiste a tu teléfono para iniciar sesión. </Typo>
        <div className="ValidteView__validate__input">
          <Input name="c1" type="number" />
          <Input name="c2" type="number" />
          <Input name="c3" type="number" />
          <Input name="c4" type="number" />
          <Input name="c5" type="number" />
        </div>

        <Button onClick={tryCode}> Ingresar </Button>
        <RetryButton onClick={resendCode} />
      </div>
    </section>
  )
}

export default ValidateView