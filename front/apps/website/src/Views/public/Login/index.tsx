import { useState } from "react";
import { useNavigate } from "react-router";
import { validate } from "@/validations";

import Typo from "@repo/ui/components/typography";
import Divider from "@repo/ui/components/divider";
import Input from "@repo/ui/components/input";
import Button from "@repo/ui/components/button";
import { Link } from "react-router";

import Estampa3 from "@/assets/estampa-3.png";
import "./style.css";

const LoginView: React.FunctionComponent = () => {
  const navigate = useNavigate();
  const [error, setError] = useState<string | undefined>();
  const [loading, setLoading] = useState<boolean>(false);

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    setLoading(true);
    e.preventDefault();
    setError(undefined);

    validate(
      "telephone",
      (e.currentTarget.elements.namedItem("telephone") as HTMLInputElement)
        .value,
    )
      .then((res) => {
        console.log(res);
        navigate("/validate");
      })
      .catch((e) => setError(e))
      .finally(() => setLoading(false));
  };

  return (
    <section className="LoginView">
      <div className="LoginView__head">
        <Typo align="center" type="title">
          Iniciar sesión
        </Typo>
        <Divider />
        <img className="LoginView__estampa" src={Estampa3} alt="inici-sesion" />
      </div>

      <form onSubmit={onSubmit} autoComplete="off">
        <div className="LoginView__login">
          <Typo>
            Es bueno tenerte de vuelta, ten a la mano tu teléfono para poder
            recibir e ingresar el código posteriormente.
          </Typo>
          <Input
            name="telephone"
            type="tel"
            placeholder="Teléfono de Whatsapp"
            error={error}
          />
          <Button loading={loading} type="submit" > Ingresar </Button>
          <Link to="/">
            <Button variant="text"> Regresar </Button>
          </Link>
        </div>
      </form>
    </section>
  );
};

export default LoginView;
