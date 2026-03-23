
export interface QueryResult {
    columns: string[];
    columnTypes?: string[];
    rows: any[][];
    executionTime?: number;
    affectedRows?: number;
    error?: string;
}


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
    const res = await fetch("./api/tables", {
        headers: { "x-api-key": apiKey }
    });
    await handleResponse(res, onUnauthorized);
    if (res.ok) {
        return await res.json();
    }
    throw new Error("Failed to fetch tables");
};

export const execSql = async (sql: string, apiKey: string, onUnauthorized?: () => void): Promise<QueryResult> => {
    const response = await fetch("./api/query", {
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

export interface CorrectResult {
    sql?: string;
    error?: string;
}

export const correctSql = async (sql: string, table: string, apiKey: string, onUnauthorized?: () => void): Promise<CorrectResult> => {
    const response = await fetch("./api/correct", {
        method: "POST",
        headers: getHeaders(apiKey),
        body: JSON.stringify({ sql, table }),
    });

    await handleResponse(response, onUnauthorized);
    return await response.json();
};

export const getDbFiles = async (apiKey: string, onUnauthorized?: () => void): Promise<string[]> => {
    const res = await fetch("./api/db-files", {
        headers: { "x-api-key": apiKey }
    });
    await handleResponse(res, onUnauthorized);
    if (res.ok) {
        return await res.json();
    }
    throw new Error("Failed to fetch db files");
};

export const connectDb = async (
    path: string,
    create: boolean,
    apiKey: string,
    onUnauthorized?: () => void
): Promise<{ success: boolean; status: number }> => {
    const res = await fetch("./api/connect", {
        method: "POST",
        headers: getHeaders(apiKey),
        body: JSON.stringify({ path, create }),
    });

    if (res.status === 401) {
        if (onUnauthorized) onUnauthorized();
        return { success: false, status: 401 };
    }
    return { success: res.ok, status: res.status };
};
