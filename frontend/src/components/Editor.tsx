import { Component, createEffect, createSignal } from "solid-js";
import {
  setSqlContent,
  setShouldFocusSqlInput,
  shouldFocusSqlInput,
  sqlContent,
  apiKey,
} from "../lib/store";
import { aiCorrectSql } from "../lib/api";

const Editor: Component = () => {
  let textareaRef: HTMLTextAreaElement | undefined;
  const [isAILoading, setIsAILoading] = createSignal(false);

  createEffect(() => {
    if (shouldFocusSqlInput() && textareaRef) {
      textareaRef.focus();
      setShouldFocusSqlInput(false);
    }
  });

  const handleAICorrect = async () => {
    const currentSql = sqlContent();
    if (!currentSql.trim()) return;

    setIsAILoading(true);
    try {
      const result = await aiCorrectSql(currentSql, apiKey());
      if (result.error) {
        alert("AI Correction Error: " + result.error);
      } else {
        setSqlContent(result.sql);
      }
    } catch (e) {
      console.error(e);
      alert("Failed to connect to AI service");
    } finally {
      setIsAILoading(false);
    }
  };

  return (
    <div>
      <div class="flex justify-between items-center mb-2">
        <h2 class="text-lg font-bold">SQL Editor</h2>
        <button 
          class="btn btn-xs btn-outline btn-primary" 
          onClick={handleAICorrect}
          disabled={isAILoading()}
        >
          {isAILoading() ? "Correcting..." : "🪄 AI Correct"}
        </button>
      </div>
      <textarea
        ref={textareaRef}
        class="textarea textarea-bordered w-full font-mono h-24"
        placeholder="SELECT * FROM sqlite_master"
        value={sqlContent()}
        onInput={(e) => setSqlContent(e.currentTarget.value)}
      ></textarea>
    </div>
  );
};

export default Editor;
