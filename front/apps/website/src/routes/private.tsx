import { BrowserRouter, Routes, Route, Navigate, useLocation } from "react-router";

// Private views
import MyFriend from "@/modules/user/app/Views/MyFriend";
import FillProfile from "@/modules/user/app/Views/FillProfile";
import FillAddress from "@/modules/user/app/Views/FillAddress";

// Legal Views
import PrivacyNotice from "@/modules/legal/app/Views/PrivacyNotice";
import TermsAndConditions from "@/modules/legal/app/Views/TermsAndConditions";


import CataloguesProvider from "@/modules/catalogues/app/CataloguesProvider";
import { useUser } from "@/modules/user/app/UserProvider";

const RequireProfile: React.FunctionComponent<{ children: React.ReactNode }> = ({ children }) => {
  const { user } = useUser()
  const location = useLocation()

  if (user.username === null && location.pathname !== "/fill-profile") {
    return <Navigate to="/fill-profile" replace />
  }

  return <>{children}</>
}

const UserRoutes: React.FunctionComponent = () => {
  return (
    <CataloguesProvider>
      <BrowserRouter>
        <RequireProfile>
          <Routes>
            <Route path="/" element={<MyFriend />} />
            <Route path="/fill-profile" element={<FillProfile />} />
            <Route path="/fill-address" element={<FillAddress />} />
            <Route path="/aviso-de-privacidad" element={<PrivacyNotice />} />
            <Route path="/terminos-y-condiciones" element={<TermsAndConditions />} />

          </Routes>
        </RequireProfile>
      </BrowserRouter>
    </CataloguesProvider>
  )
}

export default UserRoutes