import { Component } from "solid-js";

const Editor: Component = () => {
  return (
    <div>
      <h2 class="text-lg font-bold mb-2">SQL Editor</h2>
      <textarea
        class="textarea textarea-bordered w-full font-mono"
        placeholder="select * from sqlite_master"
      ></textarea>
    </div>
  );
};

export default Editor;
