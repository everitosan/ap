import { BrowserRouter, Routes, Route } from "react-router";

// Private views
import FillProfile from "@/modules/user/app/Views/FillProfile";


const UserRoutes: React.FunctionComponent = () => {
  return (
    <BrowserRouter>
        <Routes>
          <Route path="/fill-profile" element={<FillProfile />} />
        </Routes>
      </BrowserRouter>
  )
}

export default UserRoutes