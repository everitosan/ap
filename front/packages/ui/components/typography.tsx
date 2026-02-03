import "../styles/components/typography.css"


const getClassName = (type: string, align: string) => (`typography typography--${align} typography--${type} `)

const Typography: React.FunctionComponent<{
  children: React.ReactNode
  type?: "title" | "paragraph" | "section"
  align?: "left" | "center" | "right"
}> = ({ children, type = "paragraph", align="left" }) => {

  if (type === "title") return <h1 className={getClassName(type, align)}> {children} </h1>;
  if (type === "paragraph" || type === "section" ) return <p  className={getClassName(type, align)}> {children} </p>;
};

export default Typography;
