import "../styles/components/input.css"

const Input: React.FunctionComponent<{
  placeholder?: string
  type?: "text" | "phone" | "number" 
}> = ({ placeholder = "", type = "text" }) => {
  return (
    <input 
      className="input" 
      placeholder={placeholder} 
      type={type} 
    />
  )
}

export default Input