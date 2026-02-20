import { useState } from "react"
import { Link, useNavigate } from "react-router"
import { validate } from "@/validations"
import { loginOrRegister } from "@/modules/auth/infra/repository/login-register"
import { FetchError } from "justfetch-ts"
import type { ApiError } from "@/api/ap"

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
  const [acceptedTerms, setAcceptedTerms] = useState<boolean>(false);

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);
    setError(undefined);

    const telephone = (e.currentTarget.elements.namedItem("telephone") as HTMLInputElement).value;

    try {
      await validate("telephone", telephone);
      await loginOrRegister(telephone);
      navigate("/validate");
    } catch (err) {
      if (err instanceof FetchError) {
        const apiError = err.message as unknown as ApiError;
        alert(apiError.message || "Error al registrarse");
      } else if (typeof err === "string") {
        setError(err);
      } else {
        console.error(err)
        alert("Error inesperado");
      }
    } finally {
      setLoading(false);
    }
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
          <label className="HomeView__terms">
            <input
              type="checkbox"
              checked={acceptedTerms}
              onChange={(e) => setAcceptedTerms(e.target.checked)}
            />
            <span>
              Acepto los{" "}
              <Link to="/terminos-y-condiciones">Términos y Condiciones</Link> y
              el <Link to="/aviso-de-privacidad">Aviso de Privacidad</Link>
            </span>
          </label>
          <Button loading={loading} type="submit" disabled={!acceptedTerms}>
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
