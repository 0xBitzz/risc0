"use client";

import { Input } from "@risc0/ui/input";
import type { Table } from "@tanstack/react-table";
import type { ComponentType, Dispatch, SetStateAction } from "react";
import { TableViewOptions } from "~/client/table/table-view-options";
import { TableFacetedFilter } from "./table-faceted-filter";

type TableToolbarProps<TData> = {
  table: Table<TData>;
  globalFilter: string;
  statuses?: {
    label: string;
    value: string;
    icon?: ComponentType<{ className?: string }>;
  }[];
  setGlobalFilter: Dispatch<SetStateAction<string>>;
};

export function TableToolbar<TData>({ table, statuses, setGlobalFilter, globalFilter }: TableToolbarProps<TData>) {
  return (
    <div className="flex items-center justify-end gap-2">
      <Input
        placeholder="Search…"
        value={globalFilter ?? ""}
        onChange={(event) => setGlobalFilter(String(event.target.value))}
        className="h-8 w-[180px]"
      />

      {statuses && table.getColumn("status") && (
        <TableFacetedFilter column={table.getColumn("status")} title="Status" options={statuses} />
      )}

      <TableViewOptions table={table} />
    </div>
  );
}
