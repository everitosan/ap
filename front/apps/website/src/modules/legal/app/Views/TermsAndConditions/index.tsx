import { Link } from "react-router";

import Typo from "@repo/ui/components/typography";
import Divider from "@repo/ui/components/divider";
import Button from "@repo/ui/components/button";

import "./style.css";

const TermsAndConditions: React.FunctionComponent = () => {
  return (
    <section className="TermsAndConditions">
      <div className="TermsAndConditions__header">
        <Typo type="title" align="center">
          Términos y Condiciones de Uso
        </Typo>
        <Divider />
      </div>

      <div className="TermsAndConditions__content">
        <div className="TermsAndConditions__section">
          <Typo type="section">1. Identificación del responsable</Typo>
          <Typo>
            El presente documento regula el uso de la aplicación operada por
            Everardo Sanchez Hernandez, persona física con domicilio en Ciudad
            de México, México (en adelante, el "Responsable").
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">2. Aceptación expresa</Typo>
          <Typo>
            Al registrarse y utilizar la Aplicación, el usuario declara haber
            leído, entendido y aceptado plenamente los presentes Términos y
            Condiciones.
          </Typo>
          <Typo>
            Si el usuario no está de acuerdo, deberá abstenerse de utilizar la
            Aplicación.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">3. Naturaleza del servicio</Typo>
          <Typo>
            La Aplicación es una plataforma digital gratuita que facilita la
            conexión voluntaria entre usuarios para el intercambio de
            correspondencia postal.
          </Typo>
          <Typo>El Responsable:</Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>No actúa como empresa de mensajería.</Typo></li>
            <li><Typo>No interviene en el envío físico de correspondencia.</Typo></li>
            <li><Typo>No supervisa el contenido de las cartas.</Typo></li>
            <li><Typo>No verifica la identidad real de los usuarios.</Typo></li>
          </ul>
          <Typo>
            La Aplicación constituye únicamente una herramienta tecnológica de
            conexión entre particulares.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">4. Requisito de mayoría de edad</Typo>
          <Typo>
            La Aplicación está dirigida exclusivamente a personas mayores de 18
            años.
          </Typo>
          <Typo>
            El usuario declara bajo protesta de decir verdad que es mayor de
            edad y que cuenta con capacidad legal suficiente para obligarse
            conforme a estos Términos.
          </Typo>
          <Typo>
            En caso de detectarse el registro de un menor, la cuenta podrá ser
            cancelada de inmediato.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">5. Registro y autenticación</Typo>
          <Typo>El usuario deberá:</Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>Proporcionar un número telefónico válido.</Typo></li>
            <li><Typo>Validar su identidad mediante código OTP.</Typo></li>
          </ul>
          <Typo>
            El usuario es responsable de mantener el control exclusivo de su
            número telefónico y de las actividades realizadas desde su cuenta.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">
            6. Intercambio de dirección y advertencia de seguridad
          </Typo>
          <Typo>
            La naturaleza esencial del servicio implica que la dirección física
            proporcionada por el usuario será compartida con otro usuario
            participante para permitir el intercambio de correspondencia postal.
          </Typo>
          <Typo>
            <strong>Advertencia importante:</strong>
          </Typo>
          <Typo>
            Compartir una dirección física con terceros conlleva riesgos
            inherentes, incluyendo, de manera enunciativa mas no limitativa:
          </Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>Contacto no deseado.</Typo></li>
            <li><Typo>Uso indebido de la información.</Typo></li>
            <li>
              <Typo>
                Posibles conductas inapropiadas fuera de la plataforma.
              </Typo>
            </li>
          </ul>
          <Typo>Al utilizar la Aplicación, el usuario:</Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>Reconoce expresamente estos riesgos.</Typo></li>
            <li><Typo>Acepta compartir su dirección de manera voluntaria.</Typo></li>
            <li>
              <Typo>
                Asume la responsabilidad total derivada de dicha decisión.
              </Typo>
            </li>
          </ul>
          <Typo>
            El Responsable recomienda a los usuarios evaluar cuidadosamente la
            información que deciden compartir.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">7. Liberación expresa de responsabilidad</Typo>
          <Typo>
            El usuario libera expresamente al Responsable de cualquier
            responsabilidad derivada de:
          </Typo>
          <ul className="TermsAndConditions__list">
            <li>
              <Typo>
                El uso indebido de direcciones compartidas entre usuarios.
              </Typo>
            </li>
            <li>
              <Typo>
                Conductas ilícitas o inapropiadas realizadas por terceros.
              </Typo>
            </li>
            <li>
              <Typo>
                Pérdida, daño, retraso o extravío de correspondencia.
              </Typo>
            </li>
            <li>
              <Typo>Información falsa proporcionada por otros usuarios.</Typo>
            </li>
            <li>
              <Typo>
                Cualquier daño patrimonial, moral o de cualquier otra naturaleza
                que pudiera derivarse de la interacción entre usuarios.
              </Typo>
            </li>
          </ul>
          <Typo>
            El Responsable no será responsable por hechos ocurridos fuera del
            entorno tecnológico de la Aplicación.
          </Typo>
          <Typo>
            El usuario reconoce que el intercambio de correspondencia se realiza
            bajo su exclusiva responsabilidad y riesgo.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">8. Prohibiciones expresas</Typo>
          <Typo>
            Queda estrictamente prohibido utilizar la Aplicación para:
          </Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>Fines comerciales o publicitarios.</Typo></li>
            <li><Typo>Envío de propaganda política o religiosa.</Typo></li>
            <li><Typo>Recolección masiva de direcciones.</Typo></li>
            <li><Typo>Actividades de proselitismo.</Typo></li>
            <li><Typo>Envío de materiales ilícitos.</Typo></li>
            <li><Typo>Acoso, amenazas o conductas intimidatorias.</Typo></li>
            <li><Typo>Cualquier actividad contraria a la legislación mexicana.</Typo></li>
          </ul>
          <Typo>
            El incumplimiento podrá resultar en la cancelación inmediata de la
            cuenta y, en su caso, en la denuncia ante autoridades competentes.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">9. Cláusula de indemnización</Typo>
          <Typo>
            El usuario se obliga a sacar en paz y a salvo al Responsable frente
            a cualquier reclamación, demanda, procedimiento, daño o gasto
            (incluyendo honorarios legales) que surja como consecuencia de:
          </Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>El uso indebido de la Aplicación.</Typo></li>
            <li>
              <Typo>El uso indebido de la información de otros usuarios.</Typo>
            </li>
            <li><Typo>El incumplimiento de los presentes Términos.</Typo></li>
            <li>
              <Typo>
                Conductas realizadas fuera de la plataforma derivadas del
                intercambio de información.
              </Typo>
            </li>
          </ul>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">10. Cancelación de cuentas</Typo>
          <Typo>
            El Responsable podrá suspender o cancelar cuentas cuando:
          </Typo>
          <ul className="TermsAndConditions__list">
            <li><Typo>Exista incumplimiento de estos Términos.</Typo></li>
            <li>
              <Typo>Se detecte uso indebido de información personal.</Typo>
            </li>
            <li><Typo>Se identifiquen riesgos para otros usuarios.</Typo></li>
          </ul>
          <Typo>La cancelación podrá realizarse sin previo aviso.</Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">11. Protección de datos personales</Typo>
          <Typo>
            El tratamiento de datos personales se rige por el Aviso de
            Privacidad disponible en:{" "}
            <Link to="/aviso-de-privacidad">/aviso-de-privacidad</Link>.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">12. Modificaciones</Typo>
          <Typo>
            El Responsable podrá modificar los presentes Términos en cualquier
            momento. Las modificaciones serán publicadas en la misma sección.
          </Typo>
        </div>

        <div className="TermsAndConditions__section">
          <Typo type="section">13. Legislación aplicable y jurisdicción</Typo>
          <Typo>
            Para la interpretación y cumplimiento de los presentes Términos, las
            partes se someten a las leyes de México y a la jurisdicción de los
            tribunales competentes en Ciudad de México, renunciando a cualquier
            otro fuero que pudiera corresponderles.
          </Typo>
        </div>

        <div className="TermsAndConditions__footer">
          <Link to="/">
            <Button variant="text">Regresar</Button>
          </Link>
        </div>
      </div>
    </section>
  );
};

export default TermsAndConditions;
