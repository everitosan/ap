import "../styles/components/button.css"

const Button: React.FunctionComponent<{
  children: React.ReactNode
  variant?: "box" | "text"
  type?: "button" | "submit"
}> = ({ children, type="button", variant="box" }) => {
  return (
    <button 
      className={`button button--${variant}`} 
      type={type} >
      {children}
    </button>
  )
}

export default Button