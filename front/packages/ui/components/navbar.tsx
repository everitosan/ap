import Logo from "./logo"
import "../styles/components/navbar.css"

const Navbar :React.FunctionComponent<{
  showMenu?: boolean
}> = ({ showMenu = false }) => {
  return (
    <nav className="navbar">
      <Logo />

      { showMenu && <></> }

    </nav>
  )
}

export default Navbar