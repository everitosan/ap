import { BrowserRouter, Routes, Route } from "react-router";

// Public views
import Home from "@/modules/auth/app/Views/Home/index";
import Login from "@/modules/auth/app/Views/Login/index";
import Validate from "@/modules/auth/app/Views/Validate";

// Legal views
import PrivacyNotice from "@/modules/legal/app/Views/PrivacyNotice";
import TermsAndConditions from "@/modules/legal/app/Views/TermsAndConditions";

const UserRoutes: React.FunctionComponent = () => {
  return (
    <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/validate" element={<Validate />} />
          <Route path="/aviso-de-privacidad" element={<PrivacyNotice />} />
          <Route path="/terminos-y-condiciones" element={<TermsAndConditions />} />
        </Routes>
      </BrowserRouter>
  )
}

export default UserRoutes