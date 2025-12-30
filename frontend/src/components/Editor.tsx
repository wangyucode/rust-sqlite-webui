import { Component } from "solid-js";

const Editor: Component = () => {
  return (
    <div class="bg-base-100 rounded-box p-4 shadow-lg flex-1 flex flex-col">
      <h2 class="text-lg font-bold mb-2">SQL Editor</h2>
      <textarea
        class="textarea textarea-bordered w-full flex-1 font-mono"
        placeholder="select * from sqlite_master"
      ></textarea>
      <div class="flex justify-end mt-2">
        <button class="btn btn-primary">Run Query</button>
      </div>
    </div>
  );
};

export default Editor;
