import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import Navbar from "@repo/ui/components/navbar";
import Content from "@repo/ui/components/content";
// Public views
import Home from "./Views/public/Home/index";
import Login from "./Views/public/Login/index";
import Validate from "./Views/public/Validate";

// Private views
import FillProfile from "./Views/private/FillProfile";

import { BrowserRouter, Routes, Route } from "react-router";

import "@repo/ui/styles/themes.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Navbar />
    <Content>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/validate" element={<Validate />} />
          <Route path="/fill-profile" element={<FillProfile/>} />
        </Routes>
      </BrowserRouter>
    </Content>
  </StrictMode>,
);
