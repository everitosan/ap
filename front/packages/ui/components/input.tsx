import "../styles/components/input.css"

const Input: React.FunctionComponent<{
  name: string
  placeholder?: string
  type?: "text" | "tel" | "number" 
}> = ({ placeholder = "", type = "text", name }) => {
  return (
    <input 
      className="input"
      name={name}
      id={name}
      placeholder={placeholder} 
      type={type} 
    />
  )
}

export default Input