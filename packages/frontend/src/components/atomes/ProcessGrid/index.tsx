import "react-data-grid/lib/styles.css";

import "./style.scss";
import { useCallback, useEffect, useMemo, useState } from "react";

import DataGrid, { SortColumn } from "react-data-grid";

import { Process } from "../../../types/process";
import { getColor } from "../../../utils/color";
import { generateProcess } from "../../../utils/faker";

type Props = {
  processes: Process[];
  OpenProcess: (pid: number) => void;
};

export default ({ processes, OpenProcess }: Props) => {

  const createRow = () =>
    processes
      .sort((a, b) => b.cpu - a.cpu)
      .map((p) => ({
        ...p,
        cpu: p.cpu.toFixed(2),
        mem: p.mem.toFixed(2),
      }));

  const createColumns = () =>
    Object.keys(generateProcess()).map((k) => ({
      key: k,
      name: k,
      sortable: true,
      resizable: true,
      renderCell: ({ row }: { row: any }) => (
        <div
          className="grid-cell"
          style={{
            backgroundColor: getColor(
              row["cpu"] > row["mem"] ? row["cpu"] : row["mem"]
            ),
          }}
        >
          {k === "pid" ? (
            <button onClick={() => OpenProcess(row[k])}>{row[k]}</button>
          ) : k === "cpu" || k === "mem" ? (
            `${row[k]}%`
          ) : (
            row[k]
          )}
        </div>
      ),
    }));

  const [rows, setRows] = useState(createRow());
  const [columns, setColumns] = useState(createColumns());

  useEffect(() => {
    console.log(Object.keys(generateProcess()));
    setRows(createRow());
    setColumns(createColumns());
  }, [processes]);

  const [columnsOrder, setColumnsOrder] = useState((): number[] =>
    columns.map((_, index) => index)
  );

  const [sortColumns, setSortColumns] = useState<readonly SortColumn[]>([]);
  const onSortColumnsChange = useCallback((sortColumns: SortColumn[]) => {
    setSortColumns(sortColumns.slice(-1));
  }, []);

  const reorderedColumns = useMemo(() => {
    return columnsOrder.map((index) => columns[index]);
  }, [columnsOrder]);

  const sortedRows = useMemo(() => {
    if (sortColumns.length === 0) return rows;
    const { columnKey, direction } = sortColumns[0];

    let sortedRows: any = [...rows];

    switch (columnKey) {
      case "mem":
      case "cpu":
        sortedRows = sortedRows.sort((a: any, b: any) =>
          a[columnKey].localeCompare(b[columnKey])
        );
        break;
      default:
    }
    return direction === "DESC" ? sortedRows.reverse() : sortedRows;
  }, [rows, sortColumns]);

  const onColumnsReorder = (sourceKey: string, targetKey: string) => {
    setColumnsOrder((columnsOrder) => {
      const sourceColumnOrderIndex = columnsOrder.findIndex(
        (index) => columns[index].key === sourceKey
      )!;
      const targetColumnOrderIndex = columnsOrder.findIndex(
        (index) => columns[index].key === targetKey
      )!;
      const sourceColumnOrder = columnsOrder[sourceColumnOrderIndex];
      const newColumnsOrder = columnsOrder.splice(sourceColumnOrderIndex, 1);
      newColumnsOrder.splice(targetColumnOrderIndex, 0, sourceColumnOrder);
      return newColumnsOrder;
    });
  };

  return (
    <>
      <DataGrid
        style={{ width: window.innerWidth - 60 }}
        columns={reorderedColumns}
        rows={sortedRows}
        defaultColumnOptions={sortedRows.length > 0 ? {
          width: `${
            (window.innerWidth - 60) / (Object.keys(sortedRows[0]).length + 1)
          }px`,
        } : undefined}
        sortColumns={sortColumns}
        onSortColumnsChange={onSortColumnsChange}
        onColumnsReorder={onColumnsReorder}
        onSelectedRowsChange={() => {
          console.log("selected rows changed");
        }}
      />
    </>
  );
};
