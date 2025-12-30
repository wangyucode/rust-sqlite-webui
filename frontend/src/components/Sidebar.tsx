import { Component, For } from "solid-js";
import { tables } from "../lib/store";

const Sidebar: Component = () => {
  return (
    <div class="flex-shrink-0 w-24 sm:w-64 bg-base-100 rounded-box p-4 shadow-lg h-full overflow-y-auto">
      <h2 class="text-lg font-bold mb-4 px-2">Tables</h2>
      {tables().length === 0 ? (
        <div class="text-gray-500 text-sm px-2">No tables found</div>
      ) : (
        <ul class="menu w-full p-0">
          <For each={tables()}>
            {(table) => (
              <li>
                <button class="text-left w-full">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-4 h-4 mr-2"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125m-9.75 0V5.625m0 12.75v-1.5c0-.621.504-1.125 1.125-1.125m18.375 2.625V5.625m0 12.75c0 .621-.504 1.125-1.125 1.125m1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125m0 3.75h-7.5A1.125 1.125 0 0 1 12 18.375m9.75-12.75c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125m19.5 0v1.5c0 .621-.504 1.125-1.125 1.125M2.25 5.625v1.5c0 .621.504 1.125 1.125 1.125m0 0h17.25m-17.25 0h7.5c.621 0 1.125.504 1.125 1.125M3.375 8.25v1.5c0 .621.504 1.125 1.125 1.125m17.25-2.625v1.5c0 .621-.504 1.125-1.125 1.125m-17.25 0h7.5m-7.5 0c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125m1.125-3.75h17.25m-17.25 0h7.5m-7.5 0v1.5c0 .621.504 1.125 1.125 1.125m0 0h17.25m-17.25 0h7.5"
                    />
                  </svg>
                  {table}
                </button>
              </li>
            )}
          </For>
        </ul>
      )}
    </div>
  );
};

export default Sidebar;
