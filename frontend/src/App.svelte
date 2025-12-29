<script lang="ts">
import Editor from "./components/Editor.svelte";
import Navbar from "./components/Navbar.svelte";
import ResultTable from "./components/ResultTable.svelte";
import Sidebar from "./components/Sidebar.svelte";
</script>

<div class="min-h-screen bg-base-200 p-4">
  <Navbar />

  <div class="grid grid-cols-12 gap-4 h-[calc(100vh-120px)]">
    <!-- Sidebar -->
    <div class="col-span-3 h-full">
      <Sidebar />
    </div>

    <!-- Main Content -->
    <div class="col-span-9 flex flex-col gap-4 h-full">
      <!-- Editor -->
      <Editor />

      <!-- Results -->
      <ResultTable />
    </div>
  </div>
</div>
