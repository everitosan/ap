import { useState, useEffect, useRef } from "react"
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
  const [code, setCode] = useState<string[]>(["", "", "", "", ""])
  const inputRefs = useRef<(HTMLInputElement | null)[]>([])

  const resendCode = () => {
    console.log("resending")
  }

  const tryCode = () => {
    console.log("trying code", code.join(""))
  }

  const handleChange = (index: number, value: string) => {
    if (value.length > 1) {
      value = value.slice(-1)
    }

    const newCode = [...code]
    newCode[index] = value
    setCode(newCode)

    if (value && index < 4) {
      inputRefs.current[index + 1]?.focus()
    }
  }

  const handleKeyDown = (index: number, e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Backspace" && !code[index] && index > 0) {
      inputRefs.current[index - 1]?.focus()
    }
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
          {[0, 1, 2, 3, 4].map((index) => (
            <Input
              key={index}
              name={`c${index + 1}`}
              type="text"
              maxLength={1}
              value={code[index]}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => handleChange(index, e.target.value)}
              onKeyDown={(e: React.KeyboardEvent<HTMLInputElement>) => handleKeyDown(index, e)}
              ref={(el: HTMLInputElement | null) => { inputRefs.current[index] = el }}
            />
          ))}
        </div>

        <Button onClick={tryCode}> Ingresar </Button>
        <RetryButton onClick={resendCode} />
      </div>
    </section>
  )
}

export default ValidateView