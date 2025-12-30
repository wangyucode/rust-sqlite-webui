import { Component, For, createSignal } from "solid-js";
import { setShouldFocusSqlInput, setSqlContent, tables, setCurrentTable, runQuery, currentTable, execSql, fetchTables, setQueryResult } from "../lib/store";

const Sidebar: Component = () => {
  const [showDropDialog, setShowDropDialog] = createSignal(false);
  const [tableToDrop, setTableToDrop] = createSignal("");

  const handleAddTable = () => {
    setSqlContent(`CREATE TABLE table_name (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);`);
    setShouldFocusSqlInput(true);
  };

  const handleTableClick = async (table: string) => {
    setCurrentTable(table);
    setSqlContent(`SELECT * FROM ${table} LIMIT 100`);
    await runQuery();
  };

  const handleDropClick = (table: string, e: Event) => {
    e.stopPropagation();
    setTableToDrop(table);
    setShowDropDialog(true);
  };

  const confirmDrop = async () => {
    const table = tableToDrop();
    if (table) {
      try {
        await execSql(`DROP TABLE ${table}`);
        await fetchTables();
        if (currentTable() === table) {
          setCurrentTable(null);
          setSqlContent("");
          setQueryResult(null);
        }
      } catch (e) {
        console.error("Failed to drop table", e);
      }
    }
    setShowDropDialog(false);
  };

  return (
    <div class="flex-shrink-0 w-24 sm:w-64 bg-base-100 rounded-box p-4 shadow-lg h-full overflow-y-auto">
      <h2 class="text-lg font-bold px-2">Tables</h2>
      {tables().length === 0 ? (
        <div class="text-gray-500 text-sm px-2 my-2">No tables found</div>
      ) : (
        <ul class="menu w-full flex flex-col gap-1">
          <For each={tables()}>
            {(table) => (
              <li class="flex flex-row items-center justify-between">
                <button
                  class={`${currentTable() === table ? "active" : ""}`}
                  onClick={() => handleTableClick(table)}
                >
                  {table}
                </button>
                <button
                  class="btn btn-xs btn-square btn-error"
                  title="Drop"
                  onClick={(e) => handleDropClick(table, e)}
                >
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                  </svg>
                </button>
              </li>
            )}
          </For>
        </ul>
      )}
      <button
        class="btn btn-sm btn-square btn-info"
        title="Add Table"
        onClick={handleAddTable}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M12 4.5v15m7.5-7.5h-15"
          />
        </svg>
      </button>
      <dialog class="modal" classList={{ "modal-open": showDropDialog() }}>
        <div class="modal-box">
          <h3 class="font-bold text-lg">Drop Table</h3>
          <p class="py-4">
            Are you sure you want to drop table "{tableToDrop()}"? This action cannot be undone.
          </p>
          <div class="modal-action">
            <button class="btn" onClick={() => setShowDropDialog(false)}>
              Cancel
            </button>
            <button class="btn btn-error" onClick={confirmDrop}>
              Drop
            </button>
          </div>
        </div>
      </dialog>
    </div>
  );
};

export default Sidebar;
