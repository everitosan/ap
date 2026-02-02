import "../styles/components/button.css"

const Button: React.FunctionComponent<{
  children: React.ReactNode
  variant?: "box" | "text"
  type?: "button" | "submit"
  onClick?: () => void
}> = ({ children, type="button", variant="box",  onClick }) => {
  return (
    <button 
      onClick={onClick}
      className={`button button--${variant}`} 
      type={type} >
      {children}
    </button>
  )
}

export default Button