import { Component } from "solid-js";
import Editor from "./components/Editor";
import Navbar from "./components/Navbar";
import ResultTable from "./components/ResultTable";
import Sidebar from "./components/Sidebar";

const App: Component = () => {
  return (
    <div class="min-h-screen bg-base-200 p-4">
      <Navbar />

      <div class="grid grid-cols-12 gap-4 h-[calc(100vh-120px)]">
        {/* Sidebar */}
        <div class="col-span-3 h-full">
          <Sidebar />
        </div>

        {/* Main Content */}
        <div class="col-span-9 flex flex-col gap-4 h-full">
          {/* Editor */}
          <Editor />

          {/* Results */}
          <ResultTable />
        </div>
      </div>
    </div>
  );
};

export default App;
