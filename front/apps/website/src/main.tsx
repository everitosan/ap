import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import Home from "./Views/public/Home/index"
import Navbar from "@repo/ui/components/navbar"
import Content from "@repo/ui/components/content"

import "@repo/ui/styles/themes.css"

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Navbar/>
    <Content>
      <Home />
    </Content>
  </StrictMode>,
)
