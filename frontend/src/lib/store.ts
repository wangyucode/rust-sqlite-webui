import { createSignal } from "solid-js";

export const [tables, setTables] = createSignal<string[]>([]);
export const [sqlContent, setSqlContent] = createSignal<string>("");
export const [shouldFocusSqlInput, setShouldFocusSqlInput] = createSignal<boolean>(false);
export const [currentTable, setCurrentTable] = createSignal<string | null>(null);

export interface QueryResult {
    columns: string[];
    rows: any[][];
    executionTime?: number;
    error?: string;
}

export const [queryResult, setQueryResult] = createSignal<QueryResult | null>(null);
export const [isQuerying, setIsQuerying] = createSignal<boolean>(false);

export const fetchTables = async () => {
    try {
        const res = await fetch("http://localhost:3000/api/tables");
        if (res.ok) {
            setTables(await res.json());
        }
    } catch (e) {
        console.error("Failed to fetch tables", e);
        setTables([]);
    }
};

export const execSql = async (sql: string) => {
    const response = await fetch("http://localhost:3000/api/query", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ sql }),
    });

    const data = await response.json();

    if (!response.ok) {
        throw new Error(data.error || "Failed to execute query");
    }

    return data;
};

export const runQuery = async (sqlOverride?: string) => {
    const sql = sqlOverride || sqlContent();
    if (!sql.trim()) return;

    setIsQuerying(true);
    setQueryResult(null);
    const startTime = performance.now();

    try {
        const data = await execSql(sql);

        setQueryResult({
            columns: data.columns || [],
            rows: data.rows || [],
            executionTime: performance.now() - startTime,
            error: undefined
        });

        if (sql.trim().toLowerCase().match(/^\s*(create|drop)\s+table\b/)) {
            await fetchTables();
        }
    } catch (error: any) {
        setQueryResult({
            columns: [],
            rows: [],
            executionTime: performance.now() - startTime,
            error: error.message
        });
    } finally {
        setIsQuerying(false);
    }
};
