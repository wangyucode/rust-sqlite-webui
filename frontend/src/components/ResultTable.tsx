import { Component, For, Show } from "solid-js";
import { queryResult } from "../lib/store";

const ResultTable: Component = () => {
  return (
    <div class="bg-base-100 flex-1 overflow-auto flex flex-col">
      <div class="flex justify-between items-center mb-2 px-1">
        <h2 class="text-lg font-bold">Results</h2>
        <Show when={queryResult()?.executionTime}>
          <span class="text-xs text-base-content/60">
            {queryResult()?.rows.length} rows affected in {queryResult()?.executionTime?.toFixed(2)}ms
          </span>
        </Show>
      </div>

      <Show when={queryResult()?.error}>
        <div role="alert" class="alert alert-error mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
          <span>{queryResult()?.error}</span>
        </div>
      </Show>

      <div class="overflow-x-auto">
        <table class="table table-zebra table-sm border-separate border border-base-300">
          <thead>
            <tr>
              <For each={queryResult()?.columns}>
                {(col) => <th>{col}</th>}
              </For>
            </tr>
          </thead>
          <tbody>
            <For each={queryResult()?.rows}>
              {(row) => (
                <tr>
                  <For each={row}>
                    {(cell) => <td>{cell === null ? <span class="text-base-content/40 italic">null</span> : String(cell)}</td>}
                  </For>
                </tr>
              )}
            </For>
          </tbody>
        </table>
        <Show when={!queryResult() && !queryResult()?.error}>
          <div class="p-4 text-center text-base-content/60">
            Run a query to see results
          </div>
        </Show>
        <Show when={queryResult() && queryResult()?.columns.length === 0 && !queryResult()?.error}>
          <div class="p-4 text-center text-base-content/60">
            No results returned
          </div>
        </Show>
      </div>
    </div>
  );
};

export default ResultTable;
