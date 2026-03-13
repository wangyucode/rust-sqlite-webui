import { createSignal } from "solid-js";
import * as api from "./api";

// Re-export QueryResult for components to use
export type QueryResult = api.QueryResult;

export const [apiKey, setApiKey] = createSignal<string>(localStorage.getItem("api_key") || "");
export const [isAuthModalOpen, setIsAuthModalOpen] = createSignal<boolean>(false);

export const [tables, setTables] = createSignal<string[]>([]);
export const [sqlContent, setSqlContent] = createSignal<string>("");
export const [shouldFocusSqlInput, setShouldFocusSqlInput] = createSignal<boolean>(false);
export const [currentTable, setCurrentTable] = createSignal<string | null>(null);
export const [selectedRowIndices, setSelectedRowIndices] = createSignal<number[]>([]);

export const [queryResult, setQueryResult] = createSignal<QueryResult | null>(null);
export const [isQuerying, setIsQuerying] = createSignal<boolean>(false);
export const [isCorrecting, setIsCorrecting] = createSignal<boolean>(false);

const onUnauthorized = () => setIsAuthModalOpen(true);

export const resetStore = () => {
    setTables([]);
    setCurrentTable(null);
    setSqlContent("");
    setQueryResult(null);
    setSelectedRowIndices([]);
};

export const fetchTables = async () => {
    try {
        const newTables = await api.fetchTables(apiKey(), onUnauthorized);
        setTables(newTables);
        if (!currentTable() && newTables.length > 0) {
            setCurrentTable(newTables[0]);
        }
        if (currentTable()) {
            setSqlContent(`SELECT * FROM ${currentTable()} LIMIT 100`);
            await runQuery();
        }
    } catch (e) {
        console.error("Failed to fetch tables", e);
        setTables([]);
        setCurrentTable(null);
        setSqlContent("");
    }
};

export const execSql = async (sql: string) => {
    return api.execSql(sql, apiKey(), onUnauthorized);
};

export const runQuery = async (sqlOverride?: string) => {
    const sql = sqlOverride || sqlContent();
    if (!sql.trim()) return;

    setIsQuerying(true);
    setQueryResult(null);
    setSelectedRowIndices([]);
    const startTime = performance.now();

    try {
        const data = await execSql(sql);

        setQueryResult(data);

        // Simple side-effect handling: if it's a DDL or DML, refresh related data
        const lowerSql = sql.trim().toLowerCase();
        if (lowerSql.match(/^\s*(create|drop)\s+table\b/)) {
            await fetchTables();
        } else if (lowerSql.match(/^\s*(insert|update|delete)\b/) && currentTable()) {
            // Re-fetch current table data
            const selectSql = `SELECT * FROM "${currentTable()}" LIMIT 100`;
            const selectData = await execSql(selectSql);
            // Preserve execution stats from the modification query, but show new data
            setQueryResult({
                ...selectData,
                executionTime: data.executionTime,
                affectedRows: data.affectedRows,
            });
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

export const correctSql = async () => {
    const res = queryResult();
    const sql = sqlContent();
    if (!res?.error || !sql) return;

    setIsCorrecting(true);
    try {
        const result = await api.correctSql(sql, res.error, apiKey(), onUnauthorized);
        if (result.corrected_sql) {
            setSqlContent(result.corrected_sql);
            setQueryResult({
                ...res,
                error: (res.error || "") + "\n\n✨ AI Suggestion: " + (result.explanation || "No explanation provided.")
            });
        } else if (result.error) {
            setQueryResult({
                ...res,
                error: (res.error || "") + "\n\n❌ AI Error: " + result.error
            });
        }
    } catch (e: any) {
        console.error("Failed to correct SQL", e);
    } finally {
        setIsCorrecting(false);
    }
};
