import { useState } from "react"

import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Input from "@repo/ui/components/input"
import Button from "@repo/ui/components/button"
import { validate } from "@/validations"

import "./style.css"

import TagSelector, {type Tag} from "@repo/ui/components/tag-selector"

const tags: Tag[] = [
  {
    id: "1",
    display: "Arte"
  },
  {
    id: "2",
    display: "Cultura"
  }
] 

const FillProfileView : React.FunctionComponent = () => {

  const [loading, setLoading] = useState<boolean>(false)
  const [usernameErr, setUserNameErr] = useState<string|undefined>(undefined)
  
  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setUserNameErr("")
    const username = (e.currentTarget.elements.namedItem("username") as HTMLInputElement).value
    const topics = (e.currentTarget.elements.namedItem("topics") as HTMLInputElement).value
    
    try {
      const usernameValidated = await validate("username", username)
      setLoading(true)
      console.log(usernameValidated, topics)
    } catch (e) {
      setUserNameErr(e as string)
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