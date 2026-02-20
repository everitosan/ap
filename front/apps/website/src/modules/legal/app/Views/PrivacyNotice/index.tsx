import { Link } from "react-router";

import Typo from "@repo/ui/components/typography";
import Divider from "@repo/ui/components/divider";
import Button from "@repo/ui/components/button";

import "./style.css";

const PrivacyNotice: React.FunctionComponent = () => {
  return (
    <section className="PrivacyNotice">
      <div className="PrivacyNotice__header">
        <Typo type="title" align="center">
          Aviso de Privacidad
        </Typo>
        <Divider />
      </div>

      <div className="PrivacyNotice__content">
        <Typo>
          En cumplimiento con lo dispuesto por la Ley Federal de Protección de
          Datos Personales en Posesión de los Particulares, se informa lo
          siguiente:
        </Typo>

        <div className="PrivacyNotice__section">
          <Typo type="section">1. Identidad y domicilio del responsable</Typo>
          <Typo>
            Everardo Sanchez Hernandez, persona física con domicilio en Ciudad
            de México, México, es responsable del tratamiento de los datos
            personales que se recaban a través de la aplicación y sitio web
            correspondiente (en adelante, la "Aplicación").
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">2. Datos personales que se recaban</Typo>
          <Typo>Los datos personales que se recaban son:</Typo>
          <ul className="PrivacyNotice__list">
            <li><Typo>Número telefónico</Typo></li>
            <li><Typo>Dirección física</Typo></li>
          </ul>
          <Typo>No se recaban datos personales sensibles.</Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">3. Finalidades del tratamiento</Typo>
          <Typo>
            Los datos personales serán utilizados exclusivamente para las
            siguientes finalidades primarias:
          </Typo>
          <ul className="PrivacyNotice__list">
            <li>
              <Typo>
                Permitir el registro e identificación del usuario mediante
                autenticación por código de un solo uso (OTP) enviado al número
                telefónico proporcionado.
              </Typo>
            </li>
            <li>
              <Typo>Permitir el funcionamiento de la Aplicación.</Typo>
            </li>
            <li>
              <Typo>
                Facilitar el intercambio de correspondencia postal entre
                usuarios, para lo cual la dirección física podrá ser compartida
                con otro usuario participante dentro de la dinámica de la
                Aplicación.
              </Typo>
            </li>
            <li>
              <Typo>
                Contactar al usuario en caso de incidencias relacionadas con el
                servicio.
              </Typo>
            </li>
          </ul>
          <Typo>
            En caso de que el titular no esté de acuerdo con estas finalidades,
            deberá abstenerse de utilizar la Aplicación.
          </Typo>
          <Typo>
            No se utilizan los datos personales para fines publicitarios o
            comerciales.
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">
            4. Almacenamiento y encargados del tratamiento
          </Typo>
          <Typo>
            Los datos personales son almacenados en infraestructura tecnológica
            que utiliza servicios de base de datos PostgreSQL administrados
            mediante Prisma.io, actuando como encargado del tratamiento por
            cuenta del responsable.
          </Typo>
          <Typo>
            Dichos proveedores tecnológicos están obligados contractualmente a
            mantener la confidencialidad y seguridad de la información conforme
            a la legislación aplicable.
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">5. Uso de cookies</Typo>
          <Typo>
            La Aplicación utiliza exclusivamente cookies técnicas necesarias
            para mantener la sesión activa del usuario y permitir el
            funcionamiento adecuado del servicio.
          </Typo>
          <Typo>Estas cookies:</Typo>
          <ul className="PrivacyNotice__list">
            <li><Typo>No tienen fines publicitarios.</Typo></li>
            <li><Typo>No realizan seguimiento de comportamiento.</Typo></li>
            <li>
              <Typo>Expiran automáticamente después de cinco (5) días.</Typo>
            </li>
            <li>
              <Typo>
                Se eliminan antes de dicho plazo si el usuario cierra sesión.
              </Typo>
            </li>
          </ul>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">6. Derechos ARCO</Typo>
          <Typo>El titular tiene derecho a:</Typo>
          <ul className="PrivacyNotice__list">
            <li><Typo>Acceder a sus datos personales.</Typo></li>
            <li><Typo>Rectificarlos en caso de ser inexactos.</Typo></li>
            <li>
              <Typo>
                Cancelarlos cuando considere que no se requieren para las
                finalidades señaladas.
              </Typo>
            </li>
            <li>
              <Typo>
                Oponerse al tratamiento de los mismos para fines específicos.
              </Typo>
            </li>
          </ul>
          <Typo>
            Para ejercer sus derechos ARCO deberá enviar una solicitud al correo
            electrónico:
          </Typo>
          <Typo>
            <strong>eve.san.dev+911@gmail.com</strong>
          </Typo>
          <Typo>La solicitud deberá contener:</Typo>
          <ul className="PrivacyNotice__list">
            <li><Typo>Nombre del titular.</Typo></li>
            <li>
              <Typo>Descripción clara del derecho que desea ejercer.</Typo>
            </li>
            <li><Typo>Medio para comunicar la respuesta.</Typo></li>
          </ul>
          <Typo>
            La respuesta será emitida en un plazo máximo de 20 días hábiles
            conforme a la legislación aplicable.
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">7. Revocación del consentimiento</Typo>
          <Typo>
            El titular podrá revocar el consentimiento otorgado para el
            tratamiento de sus datos personales mediante solicitud enviada al
            mismo correo señalado anteriormente.
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">8. Conservación de los datos</Typo>
          <Typo>
            Los datos personales serán conservados mientras la cuenta del
            usuario permanezca activa.
          </Typo>
          <Typo>
            En caso de cancelación de la cuenta, los datos serán eliminados
            dentro de un plazo razonable, salvo que exista obligación legal de
            conservación.
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">9. Uso exclusivo para mayores de edad</Typo>
          <Typo>
            La Aplicación no está dirigida a menores de 18 años.
          </Typo>
          <Typo>
            Al registrarse, el usuario declara bajo protesta de decir verdad que
            es mayor de edad. En caso de detectar el registro de un menor, se
            procederá a la cancelación de la cuenta y eliminación de los datos
            personales correspondientes.
          </Typo>
        </div>

        <div className="PrivacyNotice__section">
          <Typo type="section">10. Cambios al aviso de privacidad</Typo>
          <Typo>
            El responsable se reserva el derecho de modificar el presente Aviso
            de Privacidad en cualquier momento. Cualquier cambio será publicado
            en el mismo medio donde se encuentre disponible este Aviso.
          </Typo>
        </div>

        <div className="PrivacyNotice__footer">
          <Link to="/">
            <Button variant="text">Regresar</Button>
          </Link>
        </div>
      </div>
    </section>
  );
};

export default PrivacyNotice;
