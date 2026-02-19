import { useState } from "react";
import { useNavigate } from "react-router";
import { validate } from "@/validations";
import { loginOrRegister } from "@/modules/auth/infra/repository/login-register";
import { FetchError } from "justfetch-ts";
import type { ApiError } from "@/api/ap";

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
        alert(apiError.message || "Error al iniciar sesión");
      } else if (typeof err === "string") {
        setError(err);
      } else {
        alert("Error inesperado");
      }
    } finally {
      setLoading(false);
    }
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
