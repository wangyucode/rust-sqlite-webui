<script lang="ts">
  import { onMount } from "svelte";

  let isDark = false;

  // DB Logic
  let dbPaths: string[] = [];
  let currentPath = "";
  let inputPath = "";
  let isLoading = false;
  let showInput = false;
  let errorMsg = "";

  const API_URL = "http://localhost:3000/api/connect";

  onMount(async () => {
    // Theme Logic
    const theme = localStorage.getItem("theme");
    if (theme === "dark") {
      isDark = true;
      document.documentElement.setAttribute("data-theme", "dark");
    } else if (theme === "light") {
      isDark = false;
      document.documentElement.setAttribute("data-theme", "light");
    }

    // DB Auto-load Logic
    loadPaths();
    if (dbPaths.length > 0) {
      await autoLoad();
    } else {
      showInput = true;
    }
  });

  function loadPaths() {
    const stored = localStorage.getItem("db-paths");
    if (stored) {
      try {
        dbPaths = JSON.parse(stored);
      } catch {
        dbPaths = [];
      }
    }
  }

  function savePaths() {
    localStorage.setItem("db-paths", JSON.stringify(dbPaths));
  }

  async function connectAPI(path: string): Promise<boolean> {
    try {
      const res = await fetch(API_URL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ path }),
      });
      return res.ok;
    } catch (e) {
      console.error("Connection failed", e);
      return false;
    }
  }

  async function autoLoad() {
    if (dbPaths.length === 0) {
      showInput = true;
      return;
    }

    isLoading = true;
    const path = dbPaths[0];
    const success = await connectAPI(path);
    isLoading = false;

    if (success) {
      currentPath = path;
      showInput = false;
    } else {
      // Failed (404/Error), remove this path and try next
      dbPaths.shift();
      dbPaths = dbPaths; // Update reactivity
      savePaths();
      await autoLoad(); // Recursive call
    }
  }

  async function handleManualSubmit() {
    if (!inputPath) return;
    isLoading = true;
    errorMsg = "";

    const success = await connectAPI(inputPath);
    isLoading = false;

    if (success) {
      // Add to top of list
      dbPaths = dbPaths.filter((p) => p !== inputPath);
      dbPaths.unshift(inputPath);
      dbPaths = dbPaths;
      savePaths();

      currentPath = inputPath;
      showInput = false;
      inputPath = "";
    } else {
      errorMsg = "Failed to load database. Please check the path.";
    }
  }

  async function handleSelectorChange() {
    if (!currentPath) return;
    isLoading = true;
    const success = await connectAPI(currentPath);
    isLoading = false;

    if (success) {
      // Promote to top
      dbPaths = dbPaths.filter((p) => p !== currentPath);
      dbPaths.unshift(currentPath);
      dbPaths = dbPaths;
      savePaths();
    } else {
      // Failed (e.g. file moved), treat as autoLoad failure
      dbPaths = dbPaths.filter((p) => p !== currentPath);
      dbPaths = dbPaths;
      savePaths();
      await autoLoad();
    }
  }

  function switchToInput() {
    showInput = true;
    inputPath = "";
    errorMsg = "";
  }

  function toggleTheme() {
    const theme = isDark ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }
</script>

<div
  class="navbar rounded-box bg-base-100 shadow-lg mb-4 gap-4 px-4 justify-between"
>
  <h1 class="navbar-start text-xl font-bold w-auto text-primary">
    SQLite WebUI
  </h1>
  <div class="navbar-center flex-1 max-w-128 flex flex-col">
    <div class="join w-full">
      {#if showInput}
        <input
          type="text"
          placeholder="SQLite file path (e.g. /path/to/your.db)"
          class="input input-bordered w-full join-item"
          bind:value={inputPath}
          on:keydown={(e) => e.key === "Enter" && handleManualSubmit()}
        />
        <button
          class="btn btn-primary join-item"
          title="Load Database"
          on:click={handleManualSubmit}
          disabled={isLoading}
        >
          {#if isLoading}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
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
                d="M9 8.25H7.5a2.25 2.25 0 0 0-2.25 2.25v9a2.25 2.25 0 0 0 2.25 2.25h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25H15M9 12l3 3m0 0 3-3m-3 3V2.25"
              />
            </svg>
          {/if}
        </button>
      {:else}
        <select
          class="select select-bordered w-full join-item"
          bind:value={currentPath}
          on:change={handleSelectorChange}
          disabled={isLoading}
        >
          {#each dbPaths as path}
            <option value={path}>{path}</option>
          {/each}
        </select>
        <button
          class="btn btn-secondary join-item"
          on:click={switchToInput}
          title="Open New Database"
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
      {/if}
    </div>
    {#if errorMsg && showInput}
      <span class="text-error text-xs mt-1 ml-1">{errorMsg}</span>
    {/if}
  </div>
  <div class="navbar-end w-auto">
    <label class="swap swap-rotate btn btn-circle">
      <!-- this hidden checkbox controls the state -->
      <input
        type="checkbox"
        class="theme-controller"
        value="dark"
        bind:checked={isDark}
        on:change={toggleTheme}
      />

      <!-- sun icon -->
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="swap-on size-6"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
        />
      </svg>

      <!-- moon icon -->
      <svg
        class="swap-off fill-current size-6"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
      >
        <path
          d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
        />
      </svg>
    </label>
  </div>
</div>
