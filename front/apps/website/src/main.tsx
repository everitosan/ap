import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import Navbar from "@repo/ui/components/navbar";
import Content from "@repo/ui/components/content";
import Routes from "./routes";

import "@repo/ui/styles/themes.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Navbar />
    <Content>
      <Routes />
    </Content>
  </StrictMode>,
);
