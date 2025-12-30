import { Component } from "solid-js";

const ResultTable: Component = () => {
  return (
    <div class="bg-base-100 flex-1 overflow-auto">
      <h2 class="text-lg font-bold mb-2">Results</h2>
      <table class="table table-zebra table-sm border-separate border border-base-300">
        <thead>
          <tr>
            <th>Column 1</th>
            <th>Column 2</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Row 1, Column 1</td>
            <td>Row 1, Column 2</td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default ResultTable;
