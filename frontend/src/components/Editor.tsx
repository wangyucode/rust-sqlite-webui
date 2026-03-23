import { Component, createEffect, Show } from "solid-js";
import {
  correctSql,
  isCorrecting,
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
      <div class="relative">
        <textarea
          ref={textareaRef}
          class="textarea textarea-bordered w-full font-mono h-24"
          placeholder="SELECT * FROM sqlite_master"
          value={sqlContent()}
          onInput={(e) => setSqlContent(e.currentTarget.value)}
        ></textarea>
        <Show when={sqlContent().trim()}>
          <button
            class="btn btn-sm btn-primary absolute bottom-2 right-2"
            onClick={correctSql}
            disabled={isCorrecting()}
          >
            <Show when={isCorrecting()} fallback={<span>✨ AI Fix</span>}>
              <span class="loading loading-spinner loading-sm"></span>
            </Show>
          </button>
        </Show>
      </div>
    </div>
  );
};

export default Editor;
