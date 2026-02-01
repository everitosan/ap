import "../styles/components/content.css"

const Content: React.FunctionComponent<{
  children: React.ReactNode
}> = ({ children }) => {
  return (
    <div className="content">
    {children}
  </div>
  )
}

export default Content