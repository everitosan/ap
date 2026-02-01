import "../styles/components/title.css"


const Title: React.FunctionComponent<{
  children: React.ReactNode,
  
}> = ({ children }) => {
  return (
    <h1 className={`title`} >
      {children}
    </h1>
  )
}

export default Title