import "react-data-grid/lib/styles.css";

import "./style.scss";
import { useState } from "react";

import DataGrid from "react-data-grid";

import { Process } from "../../../types/process";
import { getColor } from "../../../types/color";

type Props = {
  processes: Array<Process>;
  OpenProcess: (pid: number) => void;
};

export default ({ processes, OpenProcess }: Props) => {
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
          sortable: true,
          renderCell: ({ row }: { row: any }) => (
            <div
              className="grid-cell"
              style={{
                backgroundColor: getColor(
                  row["cpu"] > row["mem"] ? row["cpu"] : row["mem"]
                ),
              }}
            >
              {k === "name" ? (
                <button onClick={() => OpenProcess(row["pid"])}>
                  {row[k]}
                </button>
              ) : (
                row[k]
              )}
            </div>
          ),
        }))}
        rows={processes.sort((a, b) => b.cpu - a.cpu)}
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
