import { SWRConfig } from "swr";
import { MainPage } from "./pages";

function App() {
  return (
    <>
      <SWRConfig value={{ revalidateOnFocus: true }}>
        <MainPage />
      </SWRConfig>
    </>
  );
}

export default App;
