
export interface QueryResult {
    columns: string[];
    columnTypes?: string[];
    rows: any[][];
    executionTime?: number;
    affectedRows?: number;
    error?: string;
}

const API_BASE = "http://localhost:3000";

const getHeaders = (apiKey: string) => ({
    "Content-Type": "application/json",
    "x-api-key": apiKey,
});

const handleResponse = async (res: Response, onUnauthorized?: () => void) => {
    if (res.status === 401) {
        if (onUnauthorized) onUnauthorized();
        throw new Error("Unauthorized");
    }
    return res;
};

export const fetchTables = async (apiKey: string, onUnauthorized?: () => void): Promise<string[]> => {
    const res = await fetch(`${API_BASE}/api/tables`, {
        headers: { "x-api-key": apiKey }
    });
    await handleResponse(res, onUnauthorized);
    if (res.ok) {
        return await res.json();
    }
    throw new Error("Failed to fetch tables");
};

export const execSql = async (sql: string, apiKey: string, onUnauthorized?: () => void): Promise<QueryResult> => {
    const response = await fetch(`${API_BASE}/api/query`, {
        method: "POST",
        headers: getHeaders(apiKey),
        body: JSON.stringify({ sql }),
    });

    await handleResponse(response, onUnauthorized);
    const data = await response.json();

    if (!response.ok) {
        throw new Error(data.error || "Failed to execute query");
    }

    // Map backend response to QueryResult
    return {
        columns: data.columns || [],
        columnTypes: data.column_types || [],
        rows: data.rows || [],
        executionTime: data.execution_time,
        affectedRows: data.affected_rows,
        error: data.error
    };
};
