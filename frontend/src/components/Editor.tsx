import { Component, createEffect } from "solid-js";
import {
  setSqlContent,
  setShouldFocusSqlInput,
  shouldFocusSqlInput,
  sqlContent,
} from "../lib/store";

const Editor: Component = () => {
  let textareaRef: HTMLTextAreaElement | undefined;

  createEffect(() => {
    if (shouldFocusSqlInput() && textareaRef) {
      textareaRef.focus();
      setShouldFocusSqlInput(false);
    }
  });

  return (
    <div>
      <h2 class="text-lg font-bold mb-2">SQL Editor</h2>
      <textarea
        ref={textareaRef}
        class="textarea textarea-bordered w-full font-mono h-24"
        placeholder="select * from sqlite_master"
        value={sqlContent()}
        onInput={(e) => setSqlContent(e.currentTarget.value)}
      ></textarea>
    </div>
  );
};

export default Editor;
