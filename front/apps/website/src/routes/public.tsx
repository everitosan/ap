import { BrowserRouter, Routes, Route } from "react-router";

// Public views
import Home from "@/modules/auth/app/Views/Home/index";
import Login from "@/modules/auth/app/Views/Login/index";
import Validate from "@/modules/auth/app/Views/Validate";

const UserRoutes: React.FunctionComponent = () => {
  return (
    <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/validate" element={<Validate />} />
        </Routes>
      </BrowserRouter>
  )
}

export default UserRoutes