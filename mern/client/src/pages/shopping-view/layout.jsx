import { Outlet } from "react-router-dom";
import ShoppingViewHeader from "./header";

export default function ShoppingLayout() {
  return (
    <div className="flex flex-col bg-white overflow-hidden">
      <main className="flex flex-col w-full">
        <ShoppingViewHeader />
        <Outlet />
      </main>
    </div>
  );
}
