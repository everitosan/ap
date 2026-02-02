import "../styles/components/button.css"

const Button: React.FunctionComponent<{
  children: React.ReactNode
  variant?: "box" | "text"
  type?: "button" | "submit"
  onClick?: () => void
  loading?: boolean
  disabled?: boolean
}> = ({ children, type = "button", variant = "box", onClick, loading, disabled }) => {
  return (
    <button
      onClick={onClick}
      className={`button button--${variant} ${loading ? "button--loading" : ""}`}
      type={type}
      disabled={disabled || loading}
    >
      {loading ? "Cargando..." : children}
    </button>
  )
}

export default Button