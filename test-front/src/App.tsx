import { useRoutes } from "react-router-dom";

import routes from "./Routes";

function App() {
  const routing = useRoutes(routes);

  return <>{routing}</>;
}

export default App;
