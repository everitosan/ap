import PublicRoutes from "./public";
import PrivateRoutes from "./private";

const Routes: React.FunctionComponent = () => {
  const has_session = () => false
  
  if (has_session()) {
    return <PrivateRoutes />
  }
  
  return <PublicRoutes />
}

export default Routes