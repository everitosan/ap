import { forwardRef } from "react"
import "../styles/components/input.css"

interface InputProps {
  name: string
  placeholder?: string
  type?: "text" | "tel" | "number"
  error?: string
  value?: string
  maxLength?: number
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void
  onKeyDown?: (e: React.KeyboardEvent<HTMLInputElement>) => void
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ placeholder = "", type = "text", name, error, value, maxLength, onChange, onKeyDown }, ref) => {
    return (
      <div className="input-wrapper">
        <input
          ref={ref}
          className={`input ${error ? "input--error" : ""}`}
          name={name}
          id={name}
          placeholder={placeholder}
          type={type}
          value={value}
          maxLength={maxLength}
          onChange={onChange}
          onKeyDown={onKeyDown}
        />
        {error && <span className="input-error">{error}</span>}
      </div>
    )
  }
)

export default Input