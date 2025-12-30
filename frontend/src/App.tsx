import { Component } from "solid-js";
import Editor from "./components/Editor";
import Navbar from "./components/Navbar";
import ResultTable from "./components/ResultTable";
import Sidebar from "./components/Sidebar";
import Toolbar from "./components/Toolbar";

const App: Component = () => {
  return (
    <div class="h-dvh bg-base-200 flex flex-col">
      <Navbar />

      <div class="flex-1 flex gap-2 sm:gap-4 p-2 sm:p-4 min-h-0">
        <Sidebar />
        {/* Main Content */}
        <div class="flex-1 min-w-0 min-h-0 flex flex-col p-2 sm:p-4 gap-2 sm:gap-4 h-full rounded-box shadow-lg bg-base-100">
          <Editor />
          <Toolbar />
          <ResultTable />
        </div>
      </div>
    </div>
  );
};

export default App;
