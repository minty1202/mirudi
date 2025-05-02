import { Button, DiffViewer } from "./components/ui";

function App() {

  return (
    <>
      <DiffViewer.Provider>
        <Button variant="default" size="md">
          Default Button
        </Button>
        <h1 className=" font-bold underline">Hello world!</h1>
      </DiffViewer.Provider>
    </>
  );
}

export default App;
