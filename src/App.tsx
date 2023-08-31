import { RouteConfig, createBrowserRouter } from "found";
import AboutPage from "./AboutPage";
import NewPositionPage from "./NewPositionPage";
import IntroPage from "./IntroPage";
import ManagePositionsPage from "./ManagePositionsPage";

const routeConfig: RouteConfig = [
  { path: "/about", Component: AboutPage },
  { path: "/new-position", Component: NewPositionPage },
  { path: "/manage-positions", Component: ManagePositionsPage },
  { path: "/intro", Component: IntroPage },
  { path: "/*", Component: IntroPage },
];

const BrowserRouter = createBrowserRouter({ routeConfig });

function App() {
  return (
    <div className="flex-col h-full flex justify-center items-center font-sans bg-background">
      <BrowserRouter />
    </div>
  );
}

export default App;
