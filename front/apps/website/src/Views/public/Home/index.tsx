// Components
import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Estampa1 from "../../../assets/estampa-1.png"
import Input from "@repo/ui/components/input"
import Button from "@repo/ui/components/button"

import "./style.css"

const Home: React.FunctionComponent = () => {
  return (
    <section className="HomeView" >

      <div className="HomeView__intro">
        <Typo type="title" align="center" > 
          Amigo Postal
        </Typo>
        <Typo align="center" > ¿Quieres crear relaciones significativas a través del papel? </Typo>
        <img className="estampa1" src={Estampa1} alt="img-1" />
        <Divider />
        <Typo> Amigo postal es un sitio dónde puedes encontrar a tu póximo confidente por correo. </Typo>
        <Typo> Solo crea una cuenta y aleatoriamente se te asignará una persona para que puedan empezar a mandarse cartas. </Typo>
      </div>

      <div className="HomeView__form" >
        <Input type="phone" placeholder="Teléfono de Whatsapp" />
        <Button> ¡Empezar! </Button>
        <Button variant="text"> Ya tengo una cuenta </Button>
      </div>

    </section>
  )
}

export default Home