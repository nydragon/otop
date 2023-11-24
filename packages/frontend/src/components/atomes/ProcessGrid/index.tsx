import "react-data-grid/lib/styles.css";

import './style.scss';
import { useState } from "react";

import DataGrid from "react-data-grid";

import { Process } from "../../../interfaces/process";

type Props = {
    processes: Array<Process>;
};

export default ({ processes } : Props) => {
  const [selectedRows, setSelectedRows] = useState(
    (): ReadonlySet<number> => new Set()
  );

  if (!processes || processes.length === 0) {
    return null;
  }

  return (
    <>
      <DataGrid
        columns={Object.keys(processes[0]).map((k) => ({
          key: k,
          name: k,
        }))}
        rows={processes}
        defaultColumnOptions={{
          sortable: true,
          resizable: true,
        }}
        selectedRows={selectedRows}
        onSelectedRowsChange={setSelectedRows}
      />
    </>
  );
};
