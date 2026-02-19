import { useState } from "react"
import { useNavigate } from "react-router"
import { FetchError } from "justfetch-ts"

// Components
import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Input from "@repo/ui/components/input"
import Button from "@repo/ui/components/button"

import { validate } from "@/validations"
import { useUser } from "@/modules/user/app/UserProvider"
import { fillAddress } from "@/modules/user/infra/repository/profile"
import type { ApiError } from "@/api/ap"

import "./style.css"

const FillAddressView: React.FunctionComponent = () => {
  const navigate = useNavigate()
  const { updateUser } = useUser()

  const [loading, setLoading] = useState<boolean>(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setErrors({})

    const street = (e.currentTarget.elements.namedItem("street") as HTMLInputElement).value
    const intNumber = (e.currentTarget.elements.namedItem("intNumber") as HTMLInputElement).value
    const postalCode = (e.currentTarget.elements.namedItem("postalCode") as HTMLInputElement).value
    const state = (e.currentTarget.elements.namedItem("state") as HTMLInputElement).value
    const city = (e.currentTarget.elements.namedItem("city") as HTMLInputElement).value
    const colony = (e.currentTarget.elements.namedItem("colony") as HTMLInputElement).value

    try {
      await validate("address", {
        street,
        intNumber,
        postalCode,
        state,
        city,
        colony
      })

      setLoading(true)
      await fillAddress({
        street,
        int_number: intNumber,
        postal_code: postalCode,
        state,
        city,
        colony
      })
      updateUser({ filled_address: true })
      navigate("/")
    } catch (err) {
      if (err instanceof FetchError) {
        const apiError = err.message as unknown as ApiError
        alert(apiError.message || "Error al guardar dirección")
      } else if (typeof err === "object" && err !== null) {
        setErrors(err as Record<string, string>)
      } else {
        alert("Error inesperado")
      }
    } finally {
      setLoading(false)
    }
  }

  return (
    <section className="FillAddressView">
      <Typo type="title" align="center">Dirección</Typo>
      <Divider />
      <Typo> Necesitamos tu dirección para que tu nuevo amigo pueda comunicarse contigo vía correo postal. </Typo>
      <Typo type="section"> Dirección </Typo>

      <form className="FillAddressView__form" onSubmit={onSubmit} autoComplete="off">
        <Input
          name="street"
          placeholder="Calle y número"
          error={errors.street}
        />

        <div className="FillAddressView__form__row">
          <Input
            name="intNumber"
            placeholder="No int"
            error={errors.intNumber}
          />
          <Input
            name="postalCode"
            placeholder="CP"
            error={errors.postalCode}
          />
        </div>

        <Input
          name="state"
          placeholder="Estado/Provincia/Región"
          error={errors.state}
        />

        <Input
          name="city"
          placeholder="Ciudad"
          error={errors.city}
        />

        <Input
          name="colony"
          placeholder="Colonia"
          error={errors.colony}
        />

        <div className="FillAddressView__form__buttons">
          <Button loading={loading} type="submit">Guardar</Button>
        </div>
      </form>
    </section>
  )
}

export default FillAddressView
