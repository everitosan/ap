import Title from "./title"
import "@repo/ui/styles/components/logo.css"


const Logo: React.FunctionComponent<{
  size?: "mini" | "normal" | "big"
}> = ({ size ="mini" }) => {
  return (
    <div className={`logo logo--${size}`} >
      <Title>
        AP
      </Title>
    </div>
  )
}

export default Logo