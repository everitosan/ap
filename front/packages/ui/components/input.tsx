import "../styles/components/input.css"

const Input: React.FunctionComponent<{
  name: string
  placeholder?: string
  type?: "text" | "tel" | "number"
  error?: string
}> = ({ placeholder = "", type = "text", name, error }) => {
  return (
    <div className="input-wrapper">
      <input
        className={`input ${error ? "input--error" : ""}`}
        name={name}
        id={name}
        placeholder={placeholder}
        type={type}
      />
      {error && <span className="input-error">{error}</span>}
    </div>
  )
}

export default Input