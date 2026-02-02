import { useState } from "react"
import { Link, useNavigate } from "react-router"
import {validate} from "@/validations"
// Components
import Typo from "@repo/ui/components/typography"
import Divider from "@repo/ui/components/divider"
import Input from "@repo/ui/components/input"
import Button from "@repo/ui/components/button"

import Estampa1 from "@/assets/estampa-1.png"
import "./style.css"


const Home: React.FunctionComponent = () => {
  const navigate = useNavigate();
  const [error, setError] = useState<string | undefined>();
  const [loading, setLoading] = useState<boolean>(false);


  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    setLoading(true);
    e.preventDefault();
    setError(undefined);

    validate("telephone", (e.currentTarget.elements.namedItem("telephone") as HTMLInputElement).value)
      .then(res => {
        console.log(res)
        navigate("/validate")
      })
      .catch(e => setError(e))
      .finally(() => setLoading(false) )
  };

  return (
    <section className="HomeView">
      <div className="HomeView__intro">
        <Typo type="title" align="center">
          Amigo Postal
        </Typo>
        <Typo align="center">
          
          ¿Quieres crear relaciones significativas a través del papel?
        </Typo>
        <img className="estampa1" src={Estampa1} alt="img-1" />
      </div>
      <Divider />

      <div className="HomeView__register">
        <Typo>          
          Solo crea una cuenta y aleatoriamente se te asignará una persona para
          que puedan empezar a mandarse cartas.
        </Typo>

        <Typo>          
          Amigo postal es un sitio dónde puedes encontrar a tu póximo confidente
          por correo.
        </Typo>
        <form
          autoComplete="off"
          onSubmit={onSubmit}
          className="HomeView__register__form"
        >
          <Input
            name="telephone"
            type="tel"
            placeholder="Teléfono de Whatsapp"
            error={error}
          />
          <Button loading={loading} type="submit">
            ¡Empezar!
          </Button>
          <Link to="/login">
            <Button variant="text"> Ya tengo una cuenta </Button>
          </Link>
        </form>
      </div>
    </section>
  );
};

export default Home;
