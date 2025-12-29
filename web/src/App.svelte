<script lang="ts">
  let sql = $state('SELECT * FROM sqlite_master');
  let result = $state('');

  async function runQuery() {
    try {
      const res = await fetch('/api/query', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ sql })
      });
      const data = await res.json();
      result = JSON.stringify(data, null, 2);
    } catch (e) {
      result = 'Error: ' + e;
    }
  }
</script>

<div class="min-h-screen bg-base-200">
  <div class="navbar bg-base-100 shadow-lg mb-4">
    <a class="btn btn-ghost text-xl" href="/">SQLite WebUI</a>
  </div>

  <div class="grid grid-cols-12 gap-4 h-[calc(100vh-120px)]">
    <!-- Sidebar -->
    <div class="col-span-3 bg-base-100 rounded-box p-4 shadow-lg">
      <h2 class="text-lg font-bold mb-4">Database</h2>
      <ul class="menu bg-base-200 w-full rounded-box">
        <li><a>Tables</a></li>
        <li><a>Views</a></li>
      </ul>
    </div>

    <!-- Main Content -->
    <div class="col-span-9 flex flex-col gap-4">
      <!-- Editor -->
      <div class="bg-base-100 rounded-box p-4 shadow-lg flex-1 flex flex-col">
        <h2 class="text-lg font-bold mb-2">SQL Editor</h2>
        <textarea 
          class="textarea textarea-bordered w-full flex-1 font-mono" 
          bind:value={sql}
        ></textarea>
        <div class="flex justify-end mt-2">
          <button class="btn btn-primary" onclick={runQuery}>Run Query</button>
        </div>
      </div>

      <!-- Results -->
      <div class="bg-base-100 rounded-box p-4 shadow-lg flex-1 overflow-auto">
        <h2 class="text-lg font-bold mb-2">Results</h2>
        <pre class="bg-base-300 p-4 rounded-lg overflow-x-auto"><code>{result}</code></pre>
      </div>
    </div>
  </div>
</div>
