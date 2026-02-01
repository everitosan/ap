import "../styles/components/typography.css"


const getClassName = (align: string) => (`typography typography--${align}`)

const Typography: React.FunctionComponent<{
  children: React.ReactNode
  type?: "title" | "paragraph"
  align?: "left" | "center" | "right"
}> = ({ children, type = "paragraph", align="left" }) => {

  if (type === "title") return <h1 className={getClassName(align)}> {children} </h1>;
  if (type === "paragraph") return <p  className={getClassName(align)}> {children} </p>;
};

export default Typography;
