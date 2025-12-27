use maud::{html, Markup, DOCTYPE};
use crate::db::QueryResult;

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                link href="/static/output.css" rel="stylesheet";
            }
            body class="bg-gray-100 min-h-screen flex flex-col" {
                (content)
            }
        }
    }
}

pub fn index_page(db_path: Option<String>, tables: Vec<String>) -> Markup {
    let content = html! {
        // Navbar
        nav class="bg-blue-600 text-white p-4 shadow-md flex items-center justify-between" {
            div class="flex items-center space-x-4 flex-1" {
                span class="font-bold text-xl" { "SQLite WebUI" }
                form class="flex items-center space-x-2 flex-1 max-w-2xl" 
                      hx-post="/load-db" 
                      hx-target="#main-container"
                      hx-swap="outerHTML" {
                    input type="text" 
                          name="db_path" 
                          placeholder="Enter SQLite file path..." 
                          value=(db_path.unwrap_or_default())
                          class="bg-blue-700 text-white placeholder-blue-300 border border-blue-500 rounded px-3 py-1 flex-1 focus:outline-none focus:ring-2 focus:ring-blue-300";
                    button type="submit" 
                           class="bg-white text-blue-600 px-4 py-1 rounded font-semibold hover:bg-blue-50 transition" {
                        "Load"
                    }
                }
            }
        }

        div id="main-container" class="flex flex-1 overflow-hidden" {
            (main_content(tables, None, None))
        }
    };
    base_layout("SQLite WebUI", content)
}

pub fn main_content(tables: Vec<String>, current_table: Option<String>, query_result: Option<QueryResult>) -> Markup {
    html! {
        div id="main-container" class="flex flex-1 overflow-hidden w-full" {
            // Sidebar
            aside class="w-64 bg-white border-r border-gray-200 overflow-y-auto" {
                div class="p-4 border-b border-gray-200 font-semibold text-gray-700" { "Tables" }
                ul class="divide-y divide-gray-100" {
                    @for table in tables {
                        li {
                            button class="w-full text-left px-4 py-2 hover:bg-gray-50 text-gray-600 hover:text-blue-600 transition"
                                   hx-get=(format!("/table/{}", table))
                                   hx-target="#content-area" {
                                (table)
                            }
                        }
                    }
                }
            }

            // Main Content Area
            main id="content-area" class="flex-1 flex flex-col overflow-hidden p-6" {
                (content_area(current_table, query_result))
            }
        }
    }
}

pub fn content_area(table_name: Option<String>, result: Option<QueryResult>) -> Markup {
    html! {
        div id="content-area" class="flex-1 flex flex-col overflow-hidden" {
            // SQL Input
            div class="mb-6 bg-white p-4 rounded-lg shadow-sm" {
                h2 class="text-lg font-semibold mb-2 text-gray-700" { "Custom SQL" }
                form hx-post="/query" hx-target="#query-results" {
                    textarea name="sql" 
                             rows="4" 
                             class="w-full border border-gray-300 rounded p-2 font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none"
                             placeholder="SELECT * FROM table..." {
                        @if let Some(ref name) = table_name {
                            (format!("SELECT * FROM {} LIMIT 100", name))
                        }
                    }
                    div class="mt-2 flex justify-end" {
                        button type="submit" 
                               class="bg-blue-600 text-white px-6 py-2 rounded font-semibold hover:bg-blue-700 transition" {
                            "Execute"
                        }
                    }
                }
            }

            // Results Table
            div id="query-results" class="flex-1 bg-white rounded-lg shadow-sm overflow-hidden flex flex-col" {
                @if let Some(res) = result {
                    (results_table(res))
                } @else {
                    div class="flex-1 flex items-center justify-center text-gray-400 italic" {
                        "Execute a query or select a table to see results"
                    }
                }
            }
        }
    }
}

pub fn results_table(result: QueryResult) -> Markup {
    html! {
        div class="overflow-auto flex-1" {
            table class="min-w-full divide-y divide-gray-200" {
                thead class="bg-gray-50 sticky top-0" {
                    tr {
                        @for col in &result.columns {
                            th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" {
                                (col)
                            }
                        }
                    }
                }
                tbody class="bg-white divide-y divide-gray-200" {
                    @for row in &result.rows {
                        tr class="hover:bg-gray-50" {
                            @for val in row {
                                td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600" {
                                    (val.to_string().trim_matches('\"'))
                                }
                            }
                        }
                    }
                }
            }
        }
        div class="p-3 bg-gray-50 border-t border-gray-200 text-xs text-gray-500" {
            "Rows: " (result.rows.len())
        }
    }
}

pub fn error_message(msg: &str) -> Markup {
    html! {
        div class="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-4" role="alert" {
            p class="font-bold" { "Error" }
            p { (msg) }
        }
    }
}

pub fn success_message(msg: &str) -> Markup {
    html! {
        div class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 mb-4" role="alert" {
            p class="font-bold" { "Success" }
            p { (msg) }
        }
    }
}
