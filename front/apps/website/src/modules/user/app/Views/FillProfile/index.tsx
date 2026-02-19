import { useState } from "react"
import { useNavigate } from "react-router"
import { FetchError } from "justfetch-ts"

// Components
import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Input from "@repo/ui/components/input"
import Button from "@repo/ui/components/button"
import TagSelector from "@repo/ui/components/tag-selector"

import { validate } from "@/validations"
import { useCatalogues } from "@/modules/catalogues/app/CataloguesProvider"
import { useUser } from "@/modules/user/app/UserProvider"
import { fillProfile } from "@/modules/user/infra/repository/profile"
import type { ApiError } from "@/api/ap"

import "./style.css"


const FillProfileView : React.FunctionComponent = () => {
  const navigate = useNavigate()
  const { topics } = useCatalogues()
  const { updateUser } = useUser()
  const tags = topics.map(t => ({ id: t.id, display: t.name }))

  const [loading, setLoading] = useState<boolean>(false)
  const [usernameErr, setUserNameErr] = useState<string|undefined>(undefined)

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setUserNameErr("")
    const username = (e.currentTarget.elements.namedItem("username") as HTMLInputElement).value
    const selectedTopics = (e.currentTarget.elements.namedItem("topics") as HTMLInputElement).value

    try {
      await validate("username", username)
      setLoading(true)

      const topicsArray = selectedTopics ? selectedTopics.split(",") : []
      await fillProfile(username, topicsArray)
      updateUser({ username, topics: topicsArray })
      navigate("/")
    } catch (err) {
      if (err instanceof FetchError) {
        const apiError = err.message as unknown as ApiError
        alert(apiError.message || "Error al guardar perfil")
      } else if (typeof err === "string") {
        setUserNameErr(err)
      } else {
        alert("Error inesperado")
      }
    } finally {
      setLoading(false)
    }
  }
  
  return (
    <section className="FillProfileView">
      <Typo type="title" align="center" > Completa tu perfil </Typo>
      <Divider />
      <Typo> Define un pesudónimo, esto ayudará a que tu nuevo amigo pueda saber cómo nombrarte de inicio. </Typo>

      <form className="FillProfileView__form"  onSubmit={onSubmit} autoComplete="off">
        <Input name="username" placeholder="Pseudónimo" error={usernameErr} ></Input>
        <Typo type="section">Temas de interés </Typo>
        <TagSelector name="topics" tags={tags} />
        <div className="FillProfileView__form__buttons" >
          <Button loading={loading} type="submit" > Guardar </Button>
        </div>
      </form>

    </section>
  )
}

export default FillProfileView