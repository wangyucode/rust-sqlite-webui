import { Component, Show } from "solid-js";
import { isQuerying, runQuery, currentTable, sqlContent, setSqlContent, selectedRowIndices, execSql, setShouldFocusSqlInput, queryResult, correctSql } from "../lib/store";

const Toolbar: Component = () => {
  const handleRefresh = async () => {
    const table = currentTable();
    if (table) {
      setSqlContent(`SELECT * FROM ${table} LIMIT 100`);
      await runQuery();
    }
  };

  const handleRun = async () => {
    await runQuery();
  };

  const handleInsert = async () => {
    const table = currentTable();
    if (!table) return;

    try {
      const result = await execSql(`PRAGMA table_info("${table}")`);
      if (result.error) {
        console.error(result.error);
        return;
      }

      const nameIdx = result.columns.findIndex((c: string) => c.toLowerCase() === "name");
      if (nameIdx === -1) return;

      const columns = result.rows.map((row: any[]) => row[nameIdx]);
      const columnsStr = columns.join(", ");
      const valuesStr = columns.map(() => "''").join(", ");

      setSqlContent(`INSERT INTO "${table}" (${columnsStr})\nVALUES (${valuesStr});`);
      setShouldFocusSqlInput(true);
    } catch (e) {
      console.error("Failed to generate insert statement", e);
    }
  };

  const handleEdit = async () => {
    const table = currentTable();
    const indices = selectedRowIndices();
    const result = queryResult();

    if (!table || indices.length === 0 || !result) return;

    try {
      const info = await execSql(`PRAGMA table_info("${table}")`);
      if (info.error) {
        console.error(info.error);
        return;
      }

      const nameIdx = info.columns.findIndex((c: string) => c.toLowerCase() === "name");
      const pkIdx = info.columns.findIndex((c: string) => c.toLowerCase() === "pk");

      if (nameIdx === -1 || pkIdx === -1) return;

      const pkCols = info.rows
        .filter((r: any[]) => r[pkIdx] > 0)
        .map((r: any[]) => r[nameIdx]);

      const statements = indices.map((idx) => {
        const row = result.rows[idx];
        const setParts: string[] = [];
        const whereParts: string[] = [];

        result.columns.forEach((col, i) => {
          const val = row[i];
          const type = result.columnTypes?.[i];
          const isNull = val === null;

          let valStr: string;
          if (isNull) {
            valStr = "NULL";
          } else if (type === "BLOB") {
            valStr = String(val);
          } else if (typeof val === "number") {
            valStr = val.toString();
          } else {
            valStr = `'${String(val).replace(/'/g, "''")}'`;
          }

          setParts.push(`"${col}" = ${valStr}`);

          if (pkCols.length > 0) {
            if (pkCols.includes(col)) {
              whereParts.push(isNull ? `"${col}" IS NULL` : `"${col}" = ${valStr}`);
            }
          } else {
            whereParts.push(isNull ? `"${col}" IS NULL` : `"${col}" = ${valStr}`);
          }
        });

        return `UPDATE "${table}" SET ${setParts.join(", ")} WHERE ${whereParts.join(" AND ")};`;
      });

      setSqlContent(statements.join("\n\n"));
      setShouldFocusSqlInput(true);
    } catch (e) {
      console.error("Failed to generate update statement", e);
    }
  };

  const handleDelete = async () => {
    const table = currentTable();
    const indices = selectedRowIndices();
    const result = queryResult();

    if (!table || indices.length === 0 || !result) return;

    try {
      const info = await execSql(`PRAGMA table_info("${table}")`);
      if (info.error) {
        console.error(info.error);
        return;
      }

      const nameIdx = info.columns.findIndex((c: string) => c.toLowerCase() === "name");
      const pkIdx = info.columns.findIndex((c: string) => c.toLowerCase() === "pk");

      if (nameIdx === -1 || pkIdx === -1) return;

      const pkCols = info.rows
        .filter((r: any[]) => r[pkIdx] > 0)
        .map((r: any[]) => r[nameIdx]);

      const statements = indices.map((idx) => {
        const row = result.rows[idx];
        const whereParts: string[] = [];

        result.columns.forEach((col, i) => {
          const val = row[i];
          const type = result.columnTypes?.[i];
          const isNull = val === null;

          let valStr: string;
          if (isNull) {
            valStr = "NULL";
          } else if (type === "BLOB") {
            valStr = String(val);
          } else if (typeof val === "number") {
            valStr = val.toString();
          } else {
            valStr = `'${String(val).replace(/'/g, "''")}'`;
          }

          if (pkCols.length > 0) {
            if (pkCols.includes(col)) {
              whereParts.push(isNull ? `"${col}" IS NULL` : `"${col}" = ${valStr}`);
            }
          } else {
            whereParts.push(isNull ? `"${col}" IS NULL` : `"${col}" = ${valStr}`);
          }
        });

        return `DELETE FROM "${table}" WHERE ${whereParts.join(" AND ")};`;
      });

      setSqlContent(statements.join("\n\n"));
      setShouldFocusSqlInput(true);
    } catch (e) {
      console.error("Failed to generate delete statement", e);
    }
  };

  const handleCorrect = async () => {
    const res = queryResult();
    if (res?.error) {
      const correction = await correctSql(sqlContent(), res.error);
      if (correction.corrected_sql) {
        setSqlContent(correction.corrected_sql);
      } else if (correction.error) {
        alert(correction.error);
      }
    }
  };

  return (
    <div class="flex gap-2">
      <button
        class="btn btn-sm btn-square btn-success"
        title="Run"
        onClick={handleRun}
        disabled={isQuerying() || !sqlContent().trim()}
      >
        <Show when={!isQuerying()} fallback={<span class="loading loading-spinner loading-xs"></span>}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="size-6 stroke-current stroke-2 fill-none">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
          </svg>
        </Show>
      </button>
      <button
        class="btn btn-sm btn-square btn-primary"
        title="AI Correct"
        onClick={handleCorrect}
        disabled={isQuerying() || !queryResult()?.error}
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 10.25l-.259-1.535a1.5 1.5 0 0 0-1.226-1.226L15 7.241l1.515-.259a1.5 1.5 0 0 0 1.226-1.226L18 4.241l.259 1.515a1.5 1.5 0 0 0 1.226 1.226L21 7.241l-1.515.259a1.5 1.5 0 0 0-1.226 1.226Zm-13.504 3.414.127.751a1.5 1.5 0 0 0 1.226 1.226l.751.127-.751.127a1.5 1.5 0 0 0-1.226 1.226l-.127.751-.127-.751a1.5 1.5 0 0 0-1.226-1.226l-.751-.127.751-.127a1.5 1.5 0 0 0 1.226-1.226l.127-.751Z" />
        </svg>
      </button>
      <button
        class="btn btn-sm btn-square btn-info"
        title="Insert"
        onClick={handleInsert}
        disabled={!currentTable()}
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
      </button>
      <button
        class="btn btn-sm btn-square btn-warning"
        title="Edit"
        disabled={selectedRowIndices().length === 0}
        onClick={handleEdit}
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
        </svg>
      </button>
      <button
        class="btn btn-sm btn-square btn-error"
        title="Delete"
        disabled={selectedRowIndices().length === 0}
        onClick={handleDelete}
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
        </svg>
      </button>
      <button
        class="btn btn-sm btn-square btn-secondary"
        title="Refresh"
        onClick={handleRefresh}
        disabled={isQuerying() || !currentTable()}
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
        </svg>
      </button>
    </div>
  );
};

export default Toolbar;
